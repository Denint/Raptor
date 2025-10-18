use crate::{
    domain::{
        entities::stored_value::{get_unix_timestamp, StoredValue},
        errors::DomainError,
        value_objects::{key::Key, value::Value},
        repositories::TtlRepository,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl TtlRepository for InMemoryStorage {
    async fn set_with_ttl(
        &self,
        key: &Key,
        value: Value,
        ttl_seconds: u64,
    ) -> Result<(), DomainError> {
        let stored_value = StoredValue::with_ttl(value, ttl_seconds);
        self.store.insert(key.clone(), stored_value);
        self.check_memory_and_evict().await?;
        Ok(())
    }

    async fn set_with_ttl_ms(
        &self,
        key: &Key,
        value: Value,
        ttl_ms: u64,
    ) -> Result<(), DomainError> {
        let stored_value = StoredValue::with_ttl_ms(value, ttl_ms);
        self.store.insert(key.clone(), stored_value);
        self.check_memory_and_evict().await?;
        Ok(())
    }

    async fn expire(&self, key: &Key, ttl_seconds: u64) -> Result<bool, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                Ok(false)
            } else if ttl_seconds == 0 {
                drop(stored_value);
                self.store.remove(key);
                Ok(true)
            } else {
                stored_value.expires_at = Some(get_unix_timestamp() + ttl_seconds * 1000);
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    async fn ttl(&self, key: &Key) -> Result<Option<i64>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                Ok(Some(-2))
            } else {
                stored_value.update_access();
                match stored_value.ttl_remaining() {
                    Some(remaining) => Ok(Some(remaining as i64)),
                    None => Ok(Some(-1)),
                }
            }
        } else {
            Ok(Some(-2))
        }
    }

    async fn persist(&self, key: &Key) -> Result<bool, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                Ok(false)
            } else {
                stored_value.expires_at = None;
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::config::{Config, LogFormat};
    use std::sync::Arc;

    fn create_test_storage() -> InMemoryStorage {
        let config = Arc::new(Config {
            port: 3000,
            log_format: LogFormat::Json,
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
    async fn test_ttl_repository_set_with_ttl() {
        let storage = create_test_storage();
        let key = Key::new("ttl_key".to_string()).unwrap();
        let value = Value::Integer(42);

        storage.set_with_ttl(&key, value.clone(), 10).await.unwrap();

        let retrieved = storage.get(&key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_ttl_repository_set_with_ttl_ms() {
        let storage = create_test_storage();
        let key = Key::new("ttl_ms_key".to_string()).unwrap();
        let value = Value::String(b"test".to_vec());

        storage.set_with_ttl_ms(&key, value.clone(), 5000).await.unwrap();

        let retrieved = storage.get(&key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_ttl_repository_expire() {
        let storage = create_test_storage();
        let key = Key::new("expire_key".to_string()).unwrap();
        let value = Value::String(b"test".to_vec());

        storage.set(&key, value.clone()).await.unwrap();

        let success = storage.expire(&key, 10).await.unwrap();
        assert!(success);

        let retrieved = storage.get(&key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_ttl_repository_expire_nonexistent() {
        let storage = create_test_storage();
        let key = Key::new("nonexistent".to_string()).unwrap();

        let success = storage.expire(&key, 10).await.unwrap();
        assert!(!success);
    }

    #[tokio::test]
    async fn test_ttl_repository_ttl() {
        let storage = create_test_storage();
        let key = Key::new("ttl_test".to_string()).unwrap();
        let value = Value::String(b"test".to_vec());

        storage.set(&key, value).await.unwrap();
        let ttl = storage.ttl(&key).await.unwrap();
        assert_eq!(ttl, None);

        storage.expire(&key, 100).await.unwrap();
        let ttl = storage.ttl(&key).await.unwrap();
        assert!(ttl.is_some());
        let ttl_value = ttl.unwrap();
        assert!(ttl_value > 0 && ttl_value <= 100);
    }

    #[tokio::test]
    async fn test_ttl_repository_persist() {
        let storage = create_test_storage();
        let key = Key::new("persist_key".to_string()).unwrap();
        let value = Value::String(b"test".to_vec());

        storage.set_with_ttl(&key, value.clone(), 100).await.unwrap();

        let success = storage.persist(&key).await.unwrap();
        assert!(success);

        let ttl = storage.ttl(&key).await.unwrap();
        assert_eq!(ttl, None);

        let retrieved = storage.get(&key).await.unwrap();
        assert_eq!(retrieved, Some(value));
    }

    #[tokio::test]
    async fn test_ttl_repository_persist_nonexistent() {
        let storage = create_test_storage();
        let key = Key::new("nonexistent".to_string()).unwrap();

        let success = storage.persist(&key).await.unwrap();
        assert!(!success);
    }
}

