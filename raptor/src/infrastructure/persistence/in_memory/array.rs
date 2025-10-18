use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::ArrayRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl ArrayRepository for InMemoryStorage {
    async fn array_set(&self, key: &Key, values: Vec<Value>) -> Result<(), DomainError> {
        let stored_value = StoredValue::new(Value::Array(values));
        self.store.insert(key.clone(), stored_value);
        self.check_memory_and_evict().await?;
        Ok(())
    }

    async fn array_get(
        &self,
        key: &Key,
        indices: Vec<usize>,
    ) -> Result<Vec<Option<Value>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(vec![None; indices.len()]);
            }

            stored_value.update_access();

            if let Value::Array(ref array) = stored_value.value {
                let result: Vec<Option<Value>> = indices
                    .iter()
                    .map(|&index| {
                        if index < array.len() {
                            Some(array[index].clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(result)
            } else {
                Err(DomainError::WrongType)
            }
        } else {
            Ok(vec![None; indices.len()])
        }
    }

    async fn array_append(&self, key: &Key, values: Vec<Value>) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Err(DomainError::WrongType);
            }

            stored_value.update_access();

            if let Value::Array(ref mut array) = stored_value.value {
                array.extend(values);
                let new_len = array.len();
                Ok(new_len)
            } else {
                Err(DomainError::WrongType)
            }
        } else {
            Err(DomainError::WrongType)
        }
    }

    async fn array_slice(
        &self,
        key: &Key,
        start: usize,
        end: Option<usize>,
    ) -> Result<Vec<Value>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(Vec::new());
            }

            stored_value.update_access();

            if let Value::Array(ref array) = stored_value.value {
                let end_idx = end.unwrap_or(array.len()).min(array.len());
                if start >= array.len() || start >= end_idx {
                    Ok(Vec::new())
                } else {
                    Ok(array[start..end_idx].to_vec())
                }
            } else {
                Err(DomainError::WrongType)
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn array_update(
        &self,
        key: &Key,
        updates: Vec<(usize, Value)>,
    ) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();

            if let Value::Array(ref mut array) = stored_value.value {
                let mut updated_count = 0;
                for (index, value) in updates {
                    if index < array.len() {
                        array[index] = value;
                        updated_count += 1;
                    }
                }
                Ok(updated_count)
            } else {
                Err(DomainError::WrongType)
            }
        } else {
            Ok(0)
        }
    }

    async fn array_length(&self, key: &Key) -> Result<Option<usize>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(None);
            }

            stored_value.update_access();

            if let Value::Array(ref array) = stored_value.value {
                Ok(Some(array.len()))
            } else {
                Err(DomainError::WrongType)
            }
        } else {
            Ok(None)
        }
    }
}
