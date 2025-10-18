use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::HashRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
impl HashRepository for InMemoryStorage {
    async fn hset(&self, key: &Key, field: String, value: Vec<u8>) -> Result<bool, DomainError> {
        let is_new = {
            let mut entry = self
                .store
                .entry(key.clone())
                .or_insert_with(|| StoredValue::new(Value::Hash(HashMap::new())));

            match &mut entry.value {
                Value::Hash(hash) => Ok(hash.insert(field, value).is_none()),
                _ => Err(DomainError::WrongType),
            }
        }?;

        self.check_memory_and_evict().await?;
        Ok(is_new)
    }

    async fn hget(&self, key: &Key, field: &str) -> Result<Option<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(None);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Hash(hash) => Ok(hash.get(field).cloned()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(None)
        }
    }

    async fn hdel(&self, key: &Key, fields: Vec<String>) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &mut stored_value.value {
                Value::Hash(hash) => {
                    let mut deleted_count = 0;
                    for field in fields {
                        if hash.remove(&field).is_some() {
                            deleted_count += 1;
                        }
                    }
                    Ok(deleted_count)
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }

    async fn hexists(&self, key: &Key, field: &str) -> Result<bool, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(false);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Hash(hash) => Ok(hash.contains_key(field)),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(false)
        }
    }

    async fn hkeys(&self, key: &Key) -> Result<Vec<String>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(Vec::new());
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Hash(hash) => Ok(hash.keys().cloned().collect()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn hvals(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(Vec::new());
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Hash(hash) => Ok(hash.values().cloned().collect()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn hlen(&self, key: &Key) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Hash(hash) => Ok(hash.len()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }
}
