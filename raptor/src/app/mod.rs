use crate::infrastructure::config::Config;
use crate::infrastructure::persistence::InMemoryStorage;
use crate::infrastructure::{auditing, snapshotting};
use anyhow::Result;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, warn};
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::layer,
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
};

pub struct App {
    pub config: Arc<Config>,
    pub storage: Arc<InMemoryStorage>,
    snapshot_path: String,
    _audit_guard: Option<auditing::WorkerGuard>,
}

impl App {
    pub async fn new() -> Result<Self> {
        let config = Arc::new(Config::from_env()?);

        info!("🚀 Starting Raptor server on 0.0.0.0:{}", config.port);

        let audit_guard = Self::init_logging(&config)?;

        info!(
            port = config.port,
            log_format = ?config.log_format,
            "Application configuration loaded"
        );

        let storage = Arc::new(InMemoryStorage::new(Arc::clone(&config)));

        let snapshot_path = config
            .snapshot_path
            .as_ref()
            .unwrap_or(&"./snapshots/raptor_snapshot.bin".to_string())
            .clone();
        snapshotting::restore_snapshot_on_startup(&storage, &snapshot_path).await;

        snapshotting::setup_snapshot_mechanism(
            storage.clone(),
            config.snapshot_interval_seconds,
            snapshot_path.clone(),
        )
        .await;

        Ok(Self {
            config,
            storage,
            snapshot_path,
            _audit_guard: audit_guard,
        })
    }

    pub async fn run(self) -> Result<()> {
        let shutdown_signal = Self::create_shutdown_signal().await;
        let storage = self.storage.clone();
        let snapshot_path = self.snapshot_path.clone();

        crate::infrastructure::http::server::start_server(
            self.config,
            self.storage,
            shutdown_signal,
        )
        .await?;

        Self::save_final_snapshot_static(storage, snapshot_path).await;

        info!("Server shutdown gracefully");
        Ok(())
    }

    async fn create_shutdown_signal() -> impl std::future::Future<Output = ()> + Send {
        let ctrl_c = async {
            let _ = signal::ctrl_c().await;
        };

        #[cfg(unix)]
        let terminate = async {
            let _ = signal::unix::signal(signal::unix::SignalKind::terminate())
                .unwrap()
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        async move {
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
            info!("Shutdown signal received, starting graceful shutdown...");
        }
    }

    async fn save_final_snapshot_static(storage: Arc<InMemoryStorage>, snapshot_path: String) {
        info!("Saving final snapshot before shutdown...");
        if let Err(e) = snapshotting::create_snapshot_sync(&storage, &snapshot_path).await {
            warn!("Failed to save final snapshot: {}", e);
        } else {
            info!("Final snapshot saved successfully");
        }
    }

    fn init_logging(config: &Arc<Config>) -> anyhow::Result<Option<auditing::WorkerGuard>> {
        let audit_guard = match config.audit_log_path.as_ref() {
            Some(audit_path) => Some(auditing::init_audit_logging(audit_path.clone().into())?),
            None => None,
        };

        if audit_guard.is_none() {
            let main_log_layer = layer()
                .with_thread_ids(true)
                .with_target(true)
                .with_level(true)
                .compact()
                .boxed();

            tracing_subscriber::registry()
                .with(LevelFilter::INFO)
                .with(main_log_layer)
                .init();
        }

        Ok(audit_guard)
    }
}
