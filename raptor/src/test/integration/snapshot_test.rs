use crate::infrastructure::persistence::InMemoryStorage;
use crate::domain::{
    entities::stored_value::StoredValue,
    value_objects::{key::Key, value::Value},
    repositories::BasicRepository,
};
use crate::infrastructure::config::Config;
use std::sync::Arc;
use tokio::fs as tokio_fs;

async fn create_snapshot(
    storage: &Arc<InMemoryStorage>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let entries = storage.get_all_entries();
    let data: Vec<(String, Value)> = entries
        .into_iter()
        .map(|(key, stored_value)| (key.to_string(), stored_value.value))
        .collect();
    let serialized = serde_json::to_vec(&data)?;
    tokio_fs::write(path, serialized).await?;
    Ok(())
}

async fn restore_snapshot(
    storage: &Arc<InMemoryStorage>,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = tokio_fs::read(path).await?;
    let entries: Vec<(String, Value)> = serde_json::from_slice(&data)?;
    for (key_str, value) in entries {
        let key = Key::from(key_str);
        let stored_value = StoredValue::new(value);
        storage.insert_entry(key, stored_value);
    }
    Ok(())
}

#[tokio::test]
async fn test_snapshot_save_restore() {
    let config = Arc::new(Config::from_env().unwrap());
    let storage = Arc::new(InMemoryStorage::new(config.clone()));
    let basic_repo: Arc<dyn BasicRepository> = storage.clone();
    let snapshot_path = "test_snapshot.bin";

    let key1 = Key::from_static("test_key1");
    let value1 = Value::String(b"test_value".to_vec());
    basic_repo.set(&key1, value1.clone()).await.unwrap();

    let key2 = Key::from_static("test_key2");
    let value2 = Value::Integer(42);
    basic_repo.set(&key2, value2.clone()).await.unwrap();

    create_snapshot(&storage, snapshot_path).await.unwrap();

    let new_storage = Arc::new(InMemoryStorage::new(config));
    let new_basic_repo: Arc<dyn BasicRepository> = new_storage.clone();
    restore_snapshot(&new_storage, snapshot_path).await.unwrap();

    assert_eq!(new_basic_repo.get(&key1).await.unwrap(), Some(value1));
    assert_eq!(new_basic_repo.get(&key2).await.unwrap(), Some(value2));

    tokio_fs::remove_file(snapshot_path).await.unwrap();
}

#[tokio::test]
async fn test_snapshot_nonexistent_file() {
    let config = Arc::new(Config::from_env().unwrap());
    let storage = Arc::new(InMemoryStorage::new(config));
    let snapshot_path = "nonexistent_snapshot.bin";

    let result = restore_snapshot(&storage, snapshot_path).await;
    assert!(result.is_err());
}
