use crate::{
    domain::{
        entities::stored_value::StoredValue,
        errors::DomainError,
        value_objects::{key::Key, value::Value},
        repositories::MultiRepository,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl MultiRepository for InMemoryStorage {
    async fn mset(&self, pairs: Vec<(Key, Value)>) -> Result<(), DomainError> {
        for (key, value) in pairs {
            let stored_value = StoredValue::new(value);
            self.store.insert(key, stored_value);
        }
        self.check_memory_and_evict().await?;
        Ok(())
    }

    async fn mget(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError> {
        let mut result = Vec::with_capacity(keys.len());
        for key in keys {
            result.push(self.get(&key).await?);
        }
        Ok(result)
    }

    async fn mdel(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError> {
        let mut result = Vec::with_capacity(keys.len());
        for key in keys {
            result.push(self.delete(&key).await?);
        }
        Ok(result)
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
    async fn test_multi_repository_mset_mget() {
        let storage = create_test_storage();

        let pairs = vec![
            (Key::new("key1".to_string()).unwrap(), Value::String(b"value1".to_vec())),
            (Key::new("key2".to_string()).unwrap(), Value::Integer(42)),
            (Key::new("key3".to_string()).unwrap(), Value::String(b"value3".to_vec())),
        ];

        storage.mset(pairs.clone()).await.unwrap();

        let keys = vec![
            Key::new("key1".to_string()).unwrap(),
            Key::new("key2".to_string()).unwrap(),
            Key::new("key4".to_string()).unwrap(),
        ];

        let results = storage.mget(keys).await.unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Some(Value::String(b"value1".to_vec())));
        assert_eq!(results[1], Some(Value::Integer(42)));
        assert_eq!(results[2], None);
    }

    #[tokio::test]
    async fn test_multi_repository_mdel() {
        let storage = create_test_storage();

        let pairs = vec![
            (Key::new("del1".to_string()).unwrap(), Value::String(b"value1".to_vec())),
            (Key::new("del2".to_string()).unwrap(), Value::String(b"value2".to_vec())),
            (Key::new("del3".to_string()).unwrap(), Value::String(b"value3".to_vec())),
        ];

        storage.mset(pairs).await.unwrap();

        let keys_to_delete = vec![
            Key::new("del1".to_string()).unwrap(),
            Key::new("del2".to_string()).unwrap(),
            Key::new("nonexistent".to_string()).unwrap(),
        ];

        let deleted = storage.mdel(keys_to_delete).await.unwrap();
        assert_eq!(deleted.len(), 3);
        assert_eq!(deleted[0], Some(Value::String(b"value1".to_vec())));
        assert_eq!(deleted[1], Some(Value::String(b"value2".to_vec())));
        assert_eq!(deleted[2], None);

        let remaining = storage.get(&Key::new("del3".to_string()).unwrap()).await.unwrap();
        assert_eq!(remaining, Some(Value::String(b"value3".to_vec())));
    }
}

