use crate::{
    domain::{
        entities::stored_value::StoredValue,
        errors::DomainError,
        repositories::{BasicRepository, MultiRepository},
        value_objects::key::Key,
        value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
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
