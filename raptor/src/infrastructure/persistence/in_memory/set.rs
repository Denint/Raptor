use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::SetRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;
use std::collections::HashSet;

#[async_trait]
impl SetRepository for InMemoryStorage {
    async fn sadd(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        let added_count = {
            let mut entry = self
                .store
                .entry(key.clone())
                .or_insert_with(|| StoredValue::new(Value::Set(HashSet::new())));

            match &mut entry.value {
                Value::Set(set) => {
                    let initial_len = set.len();
                    for member in members {
                        set.insert(member);
                    }
                    Ok(set.len() - initial_len)
                }
                _ => Err(DomainError::WrongType),
            }
        }?;

        if added_count > 0 {
            self.check_memory_and_evict().await?;
        }
        Ok(added_count)
    }

    async fn smembers(&self, key: &Key) -> Result<Vec<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(Vec::new());
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Set(set) => Ok(set.iter().cloned().collect()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn srem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &mut stored_value.value {
                Value::Set(set) => {
                    let initial_len = set.len();
                    for member in members {
                        set.remove(&member);
                    }
                    Ok(initial_len - set.len())
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }

    async fn sismember(&self, key: &Key, member: &[u8]) -> Result<bool, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(false);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Set(set) => Ok(set.contains(member)),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(false)
        }
    }

    async fn scard(&self, key: &Key) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::Set(set) => Ok(set.len()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }
}
