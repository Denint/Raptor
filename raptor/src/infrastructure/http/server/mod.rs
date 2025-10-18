mod components;
mod repositories;
mod router;

pub use components::ServerComponents;
pub use repositories::Repositories;
pub use router::RouterComponents;

use crate::infrastructure::config::Config;
use crate::infrastructure::persistence::InMemoryStorage;
use std::sync::Arc;

pub async fn start_server(
    config: Arc<Config>,
    in_memory_store: Arc<InMemoryStorage>,
    shutdown_signal: impl std::future::Future<Output = ()> + Send + 'static,
) -> anyhow::Result<()> {
    let components = ServerComponents::new(in_memory_store);
    let app = components.build_router();

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to bind to address {}: {}", addr, e))?;

    tracing::info!("Server listening on http://0.0.0.0:{}", config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}
