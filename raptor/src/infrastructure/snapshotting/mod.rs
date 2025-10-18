use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use tokio::time::{Duration, interval};

use crate::domain::{
    entities::stored_value::StoredValue,
    value_objects::{key::Key, value::Value},
};
use crate::infrastructure::persistence::InMemoryStorage;
use serde_json;
use tracing::{error, info};

pub async fn setup_snapshot_mechanism(
    storage: Arc<InMemoryStorage>,
    snapshot_interval_seconds: Option<u64>,
    default_snapshot_path: String,
) {
    if let Some(interval_secs) = snapshot_interval_seconds {
        let mut interval = interval(Duration::from_secs(interval_secs));
        let storage_clone = storage.clone();
        tokio::spawn(async move {
            loop {
                interval.tick().await;
                let snapshot_path = default_snapshot_path.clone();
                info!("Attempting to create snapshot...");
                if let Err(e) = create_snapshot(&storage_clone, &snapshot_path).await {
                    error!("Failed to create snapshot: {:?}", e);
                } else {
                    info!("Snapshot created successfully at {}", snapshot_path);
                }
            }
        });
    }
}

pub async fn create_snapshot_sync(storage: &Arc<InMemoryStorage>, path: &str) -> Result<()> {
    create_snapshot(storage, path).await
}

pub async fn restore_snapshot_on_startup(storage: &Arc<InMemoryStorage>, path: &str) {
    info!("Attempting to restore snapshot from {}...", path);
    match restore_snapshot(storage, path).await {
        Ok(_) => {
            info!("Snapshot restored successfully from {}", path);
        }
        Err(e) => {
            let error_msg = format!("{}", e);
            if error_msg.contains("No such file")
                || error_msg.contains("expected value")
                || error_msg.contains("EOF while parsing")
                || error_msg.contains("invalid type")
                || error_msg.contains("missing field")
            {
                info!(
                    "No valid snapshot found at {}, starting with fresh data",
                    path
                );
            } else {
                error!("Failed to restore snapshot: {}", e);
            }
        }
    }
}

async fn create_snapshot(storage: &Arc<InMemoryStorage>, path: &str) -> Result<()> {
    let entries = storage.get_all_entries();
    let data: Vec<(String, Value)> = entries
        .into_iter()
        .map(|(key, stored_value)| (key.to_string(), stored_value.value))
        .collect();
    if let Some(parent_dir) = Path::new(path).parent() {
        tokio::fs::create_dir_all(parent_dir).await?;
    }
    let json = serde_json::to_string(&data)?;
    tokio::fs::write(path, json).await?;
    Ok(())
}

async fn restore_snapshot(storage: &Arc<InMemoryStorage>, path: &str) -> Result<()> {
    let json = tokio::fs::read_to_string(path).await?;
    if json.trim().is_empty() {
        return Ok(());
    }
    let data: Vec<(String, Value)> = serde_json::from_str(&json)?;
    for (key_str, value) in data {
        let key = Key::from(key_str);
        let stored_value = StoredValue::new(value);
        storage.insert_entry(key, stored_value);
    }
    Ok(())
}
