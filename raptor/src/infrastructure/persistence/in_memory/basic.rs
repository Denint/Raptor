use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::BasicRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
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
    async fn set(&self, key: &Key, value: Value) -> Result<bool, DomainError> {
        let stored_value = StoredValue::new(value);
        let is_new = self.store.insert(key.clone(), stored_value).is_none();
        self.check_memory_and_evict().await?;
        Ok(is_new)
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
