use crate::{
    domain::{
        entities::stored_value::{StoredValue, get_unix_timestamp},
        errors::DomainError,
        repositories::TtlRepository,
        value_objects::key::Key,
        value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
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
