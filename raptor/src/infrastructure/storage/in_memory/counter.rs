use crate::{
    domain::{
        entities::stored_value::StoredValue,
        errors::DomainError,
        value_objects::{key::Key, value::Value},
        repositories::CounterRepository,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl CounterRepository for InMemoryStorage {
    async fn incr(&self, key: &Key, increment: i64) -> Result<i64, DomainError> {
        let mut entry = self
            .store
            .entry(key.clone())
            .or_insert_with(|| StoredValue::new(Value::Integer(0)));

        entry.update_access();
        match &mut entry.value {
            Value::Integer(val) => {
                *val = val.wrapping_add(increment);
                Ok(*val)
            }
            _ => Err(DomainError::WrongType),
        }
    }

    async fn decr(&self, key: &Key) -> Result<i64, DomainError> {
        let mut entry = self
            .store
            .entry(key.clone())
            .or_insert_with(|| StoredValue::new(Value::Integer(0)));

        entry.update_access();
        match &mut entry.value {
            Value::Integer(current_value) => {
                if *current_value <= 0 {
                    return Err(DomainError::NegativeCounterValue);
                }
                *current_value = current_value.wrapping_sub(1);
                Ok(*current_value)
            }
            _ => Err(DomainError::WrongType),
        }
    }

    async fn reset(&self, key: &Key) -> Result<Option<i64>, DomainError> {
        let mut entry = self
            .store
            .entry(key.clone())
            .or_insert_with(|| StoredValue::new(Value::Integer(0)));

        entry.update_access();
        match &mut entry.value {
            Value::Integer(current_value) => {
                let previous_value = *current_value;
                *current_value = 0;
                Ok(Some(previous_value))
            }
            _ => Err(DomainError::WrongType),
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
    async fn test_counter_repository_incr() {
        let storage = create_test_storage();
        let key = Key::new("counter".to_string()).unwrap();

        let result = storage.incr(&key, 1).await.unwrap();
        assert_eq!(result, 1);

        let result = storage.incr(&key, 2).await.unwrap();
        assert_eq!(result, 3);

        let result = storage.incr(&key, -1).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_counter_repository_incr_wrong_type() {
        let storage = create_test_storage();
        let key = Key::new("wrong_type".to_string()).unwrap();
        let value = Value::String(b"not_a_number".to_vec());

        storage.set(&key, value).await.unwrap();

        let result = storage.incr(&key, 1).await;
        assert!(matches!(result, Err(DomainError::WrongType)));
    }

    #[tokio::test]
    async fn test_counter_repository_decr_success() {
        let storage = create_test_storage();
        let key = Key::new("decr_counter".to_string()).unwrap();

        storage.incr(&key, 5).await.unwrap();

        let result = storage.decr(&key).await.unwrap();
        assert_eq!(result, 4);

        let current = storage.incr(&key, 0).await.unwrap();
        assert_eq!(current, 4);
    }

    #[tokio::test]
    async fn test_counter_repository_decr_zero_value() {
        let storage = create_test_storage();
        let key = Key::new("decr_zero".to_string()).unwrap();

        let result = storage.decr(&key).await;
        assert!(matches!(result, Err(DomainError::NegativeCounterValue)));
    }

    #[tokio::test]
    async fn test_counter_repository_decr_negative_value() {
        let storage = create_test_storage();
        let key = Key::new("decr_negative".to_string()).unwrap();

        storage.incr(&key, -3).await.unwrap();

        let result = storage.decr(&key).await;
        assert!(matches!(result, Err(DomainError::NegativeCounterValue)));
    }

    #[tokio::test]
    async fn test_counter_repository_decr_wrong_type() {
        let storage = create_test_storage();
        let key = Key::new("decr_wrong_type".to_string()).unwrap();

        storage.set(&key, Value::String(b"not_a_number".to_vec())).await.unwrap();

        let result = storage.decr(&key).await;
        assert!(matches!(result, Err(DomainError::WrongType)));
    }

    #[tokio::test]
    async fn test_counter_repository_reset() {
        let storage = create_test_storage();
        let key = Key::new("reset_counter".to_string()).unwrap();

        storage.incr(&key, 5).await.unwrap();

        let old_value = storage.reset(&key).await.unwrap();
        assert_eq!(old_value, Some(5));

        let current = storage.incr(&key, 0).await.unwrap();
        assert_eq!(current, 0);
    }
}

