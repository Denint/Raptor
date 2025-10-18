use crate::{
    domain::{
        entities::stored_value::StoredValue,
        errors::DomainError,
        value_objects::{key::Key, value::Value},
        repositories::SortedSetRepository,
    },
    infrastructure::storage::in_memory::storage::InMemoryStorage,
};
use async_trait::async_trait;

#[async_trait]
impl SortedSetRepository for InMemoryStorage {
    async fn zadd(&self, key: &Key, score: f64, member: Vec<u8>) -> Result<bool, DomainError> {
        let mut entry = self.store.entry(key.clone()).or_insert_with(|| {
            StoredValue::new(Value::SortedSet(Vec::new()))
        });

        match &mut entry.value {
            Value::SortedSet(sorted_set) => {
                let existing_index = sorted_set.iter().position(|(_, m)| m == &member);

                match existing_index {
                    Some(index) => {
                        sorted_set[index] = (score, member);
                        sorted_set.sort_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        self.check_memory_and_evict().await?;
                        Ok(false)
                    }
                    None => {
                        sorted_set.push((score, member));
                        sorted_set.sort_by(|a, b| {
                            a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal)
                        });
                        self.check_memory_and_evict().await?;
                        Ok(true)
                    }
                }
            }
            _ => Err(DomainError::WrongType),
        }
    }

    async fn zrange(
        &self,
        key: &Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(Vec::new());
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::SortedSet(sorted_set) => {
                    if sorted_set.is_empty() {
                        return Ok(Vec::new());
                    }

                    let len = sorted_set.len() as isize;
                    let start_idx = if start < 0 { len + start } else { start };
                    let stop_idx = if stop < 0 { len + stop + 1 } else { stop + 1 };

                    let start_idx = start_idx.max(0).min(len) as usize;
                    let stop_idx = stop_idx.max(0).min(len) as usize;

                    if start_idx >= stop_idx {
                        return Ok(Vec::new());
                    }

                    Ok(sorted_set[start_idx..stop_idx]
                        .iter()
                        .map(|(_, member)| member.clone())
                        .collect())
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(Vec::new())
        }
    }

    async fn zrem(&self, key: &Key, members: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &mut stored_value.value {
                Value::SortedSet(sorted_set) => {
                    let initial_len = sorted_set.len();
                    sorted_set.retain(|(_, member)| !members.contains(member));
                    Ok(initial_len - sorted_set.len())
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }

    async fn zscore(&self, key: &Key, member: &[u8]) -> Result<Option<f64>, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(None);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::SortedSet(sorted_set) => {
                    Ok(sorted_set
                        .iter()
                        .find(|(_, m)| m == member)
                        .map(|(score, _)| *score))
                }
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(None)
        }
    }

    async fn zcard(&self, key: &Key) -> Result<usize, DomainError> {
        if let Some(mut stored_value) = self.store.get_mut(key) {
            if stored_value.is_expired() {
                drop(stored_value);
                self.store.remove(key);
                return Ok(0);
            }

            stored_value.update_access();
            match &stored_value.value {
                Value::SortedSet(sorted_set) => Ok(sorted_set.len()),
                _ => Err(DomainError::WrongType),
            }
        } else {
            Ok(0)
        }
    }
}

