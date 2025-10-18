use anyhow::Result;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub type WorkerGuard = tracing_appender::non_blocking::WorkerGuard;

pub fn init_audit_logging(audit_log_path: PathBuf) -> Result<WorkerGuard> {
    if audit_log_path.exists() {
        if audit_log_path.is_dir() {
            let _ = fs::remove_dir_all(&audit_log_path);
        } else {
            let _ = fs::remove_file(&audit_log_path);
        }
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(audit_log_path)
        .map_err(|e| anyhow::anyhow!("Failed to open audit log file: {}", e))?;
    let (non_blocking, guard) =
        tracing_appender::non_blocking::NonBlockingBuilder::default().finish(file);

    let audit_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .json()
        .boxed();

    let main_log_layer = tracing_subscriber::fmt::layer()
        .with_thread_ids(true)
        .with_target(true)
        .with_level(true)
        .compact()
        .boxed();

    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::INFO)
        .with(audit_layer)
        .with(main_log_layer)
        .init();

    Ok(guard)
}
