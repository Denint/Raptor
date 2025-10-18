use crate::{
    domain::{
        entities::stored_value::StoredValue, errors::DomainError, repositories::CounterRepository,
        value_objects::key::Key, value_objects::value::Value,
    },
    infrastructure::persistence::in_memory::storage::InMemoryStorage,
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
