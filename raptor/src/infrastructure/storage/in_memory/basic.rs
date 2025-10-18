use crate::{
    domain::{
        entities::stored_value::StoredValue,
        errors::DomainError,
        value_objects::{key::Key, value::Value},
        repositories::BasicRepository,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl BasicRepository for InMemoryStorage {
    #[inline]
    async fn get(&self, key: &Key) -> Result<Option<Value>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                Ok(None)
            } else {
                stored_value.update_access();
                Ok(Some(stored_value.value.clone()))
            }
        } else {
            Ok(None)
        }
    }

    #[inline]
    async fn set(&self, key: &Key, value: Value) -> Result<(), DomainError> {
        let stored_value = StoredValue::new(value);
        self.store.insert(key.clone(), stored_value);
        self.check_memory_and_evict().await?;
        Ok(())
    }

    #[inline]
    async fn delete(&self, key: &Key) -> Result<Option<Value>, DomainError> {
        if let Some((_, stored_value)) = self.store.remove(key) {
            if stored_value.is_expired() {
                Ok(None)
            } else {
                Ok(Some(stored_value.value))
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::Config;
    use std::sync::Arc;

    fn create_test_storage() -> InMemoryStorage {
        let config = Arc::new(Config {
            port: 3000,
            log_format: crate::infrastructure::config::LogFormat::Json,
            max_memory_bytes: 1024 * 1024,
            snapshot_interval_seconds: Some(10),
            audit_log_path: None,
            snapshot_path: Some("./test_snapshot.bin".to_string()),
            smart_lru_eviction_threshold: 0.8,
            smart_lru_check_interval_seconds: 30,
            smart_lru_min_eviction_ratio: 0.05,
            smart_lru_max_eviction_ratio: 0.2,
            smart_lru_ttl_weight: 0.4,
            smart_lru_access_weight: 0.4,
            smart_lru_size_weight: 0.2,
        });
        InMemoryStorage::new(config)
    }

    #[tokio::test]
    async fn test_basic_repository_get_nonexistent() {
        let storage = create_test_storage();
        let key = Key::new("nonexistent".to_string()).unwrap();

        let result = storage.get(&key).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_basic_repository_set_and_get() {
        let storage = create_test_storage();
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());

        storage.set(&key, value.clone()).await.unwrap();

        let retrieved = storage.get(&key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_basic_repository_delete() {
        let storage = create_test_storage();
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());

        storage.set(&key, value.clone()).await.unwrap();
        let deleted = storage.delete(&key).await.unwrap();
        assert_eq!(deleted, Some(value));

        let retrieved = storage.get(&key).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_basic_repository_delete_nonexistent() {
        let storage = create_test_storage();
        let key = Key::new("nonexistent".to_string()).unwrap();

        let deleted = storage.delete(&key).await.unwrap();
        assert!(deleted.is_none());
    }
}

