use std::sync::Arc;

use crate::domain::{
    errors::DomainError,
    value_objects::{key::Key, value::Value},
};

use super::array::controller::ArrayController;
use super::basic::controller::BasicController;
use super::ttl::controller::TtlController;

pub struct SingleKeyController {
    basic: Arc<BasicController>,
    ttl: Arc<TtlController>,
    array: Arc<ArrayController>,
}

impl SingleKeyController {
    pub fn new(
        basic: Arc<BasicController>,
        ttl: Arc<TtlController>,
        array: Arc<ArrayController>,
    ) -> Self {
        Self { basic, ttl, array }
    }

    pub async fn get_key(&self, key: Key) -> Result<Option<Value>, DomainError> {
        self.basic.get_key(key).await
    }

    pub async fn set_key(&self, key: Key, value: Value) -> Result<bool, DomainError> {
        self.basic.set_key(key, value).await
    }

    pub async fn delete_key(&self, key: Key) -> Result<Option<Value>, DomainError> {
        self.basic.delete_key(key).await
    }

    pub async fn set_with_ttl(
        &self,
        key: Key,
        value: Value,
        ttl_seconds: u64,
    ) -> Result<(), DomainError> {
        self.ttl.set_with_ttl(key, value, ttl_seconds).await
    }

    pub async fn ttl(&self, key: Key) -> Result<Option<i64>, DomainError> {
        self.ttl.ttl(key).await
    }

    pub async fn persist(&self, key: Key) -> Result<bool, DomainError> {
        self.ttl.persist(key).await
    }

    pub async fn expire(&self, key: Key, ttl_seconds: u64) -> Result<bool, DomainError> {
        self.ttl.expire(key, ttl_seconds).await
    }

    pub async fn array_set(&self, key: Key, values: Vec<Value>) -> Result<(), DomainError> {
        self.array.array_set(key, values).await
    }

    pub async fn array_get(
        &self,
        key: Key,
        indices: Vec<usize>,
    ) -> Result<Vec<Option<Value>>, DomainError> {
        self.array.array_get(key, indices).await
    }

    pub async fn array_append(&self, key: Key, values: Vec<Value>) -> Result<usize, DomainError> {
        self.array.array_append(key, values).await
    }

    pub async fn array_slice(
        &self,
        key: Key,
        start: usize,
        end: Option<usize>,
    ) -> Result<Vec<Value>, DomainError> {
        self.array.array_slice(key, start, end).await
    }

    pub async fn array_update(
        &self,
        key: Key,
        updates: Vec<(usize, Value)>,
    ) -> Result<usize, DomainError> {
        self.array.array_update(key, updates).await
    }

    pub async fn array_length(&self, key: Key) -> Result<Option<usize>, DomainError> {
        self.array.array_length(key).await
    }
}
