use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::ListRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl ListRepository for InMemoryStorage {
    async fn lpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        let new_len = {
            let mut entry = self
                .store
                .entry(key.clone())
                .or_insert_with(|| StoredValue::new(Value::List(Vec::new())));

            match &mut entry.value {
                Value::List(list) => {
                    for value in values {
                        list.insert(0, value);
                    }
                    Ok(list.len())
                }
                _ => Err(DomainError::WrongType),
            }
        }?;
        self.check_memory_and_evict().await?;
        Ok(new_len)
    }

    async fn rpush(&self, key: &Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        let new_len = {
            let mut entry = self
                .store
                .entry(key.clone())
                .or_insert_with(|| StoredValue::new(Value::List(Vec::new())));

            match &mut entry.value {
                Value::List(list) => {
                    for value in values {
                        list.push(value);
                    }
                    Ok(list.len())
                }
                _ => Err(DomainError::WrongType),
            }
        }?;
        self.check_memory_and_evict().await?;
        Ok(new_len)
    }

    async fn lpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(None);
            }
            stored_value.update_access();
            match &mut stored_value.value {
                Value::List(list) => {
                    if list.is_empty() {
                        Ok(None)
                    } else {
                        Ok(Some(list.remove(0)))
                    }
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(None)
        }
    }

    async fn rpop(&self, key: &Key) -> Result<Option<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(None);
            }

            stored_value.update_access();
            match &mut stored_value.value {
                Value::List(list) => Ok(list.pop()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(None)
        }
    }

    async fn lrange(
        &self,
        key: &Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(vec![]);
            }
            stored_value.update_access();
            match &stored_value.value {
                Value::List(list) => {
                    let len = list.len() as isize;
                    let mut actual_start = start;
                    let mut actual_stop = stop;

                    if actual_start < 0 {
                        actual_start += len;
                    }
                    if actual_stop < 0 {
                        actual_stop += len;
                    }

                    if actual_start < 0 {
                        actual_start = 0;
                    }
                    if actual_stop >= len {
                        actual_stop = len - 1;
                    }

                    if actual_start > actual_stop || actual_start >= len || actual_stop < 0 {
                        Ok(vec![])
                    } else {
                        Ok(list[actual_start as usize..=actual_stop as usize].to_vec())
                    }
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(vec![])
        }
    }
}
