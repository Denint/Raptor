use crate::{
    application::use_cases::hash::operations::{
        hdel_use_case::{HDelInput, HDelUseCase},
        hexists_use_case::{HExistsInput, HExistsUseCase},
        hget_use_case::{HGetInput, HGetUseCase},
        hkeys_use_case::{HKeysInput, HKeysUseCase},
        hlen_use_case::{HLenInput, HLenUseCase},
        hset_use_case::{HSetInput, HSetUseCase},
        hvals_use_case::{HValsInput, HValsUseCase},
    },
    domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

pub struct HashController {
    hset_use_case: Arc<HSetUseCase<InMemoryStorage>>,
    hget_use_case: Arc<HGetUseCase<InMemoryStorage>>,
    hdel_use_case: Arc<HDelUseCase<InMemoryStorage>>,
    hexists_use_case: Arc<HExistsUseCase<InMemoryStorage>>,
    hkeys_use_case: Arc<HKeysUseCase<InMemoryStorage>>,
    hvals_use_case: Arc<HValsUseCase<InMemoryStorage>>,
    hlen_use_case: Arc<HLenUseCase<InMemoryStorage>>,
}

impl HashController {
    pub fn new(
        hset_use_case: Arc<HSetUseCase<InMemoryStorage>>,
        hget_use_case: Arc<HGetUseCase<InMemoryStorage>>,
        hdel_use_case: Arc<HDelUseCase<InMemoryStorage>>,
        hexists_use_case: Arc<HExistsUseCase<InMemoryStorage>>,
        hkeys_use_case: Arc<HKeysUseCase<InMemoryStorage>>,
        hvals_use_case: Arc<HValsUseCase<InMemoryStorage>>,
        hlen_use_case: Arc<HLenUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            hset_use_case,
            hget_use_case,
            hdel_use_case,
            hexists_use_case,
            hkeys_use_case,
            hvals_use_case,
            hlen_use_case,
        }
    }

    pub async fn hset(&self, key: Key, field: String, value: Vec<u8>) -> Result<bool, DomainError> {
        self.hset_use_case
            .execute(HSetInput::new(key, field, value))
            .await
    }

    pub async fn hget(&self, key: Key, field: String) -> Result<Option<Vec<u8>>, DomainError> {
        self.hget_use_case.execute(HGetInput::new(key, field)).await
    }

    pub async fn hdel(&self, key: Key, fields: Vec<String>) -> Result<usize, DomainError> {
        self.hdel_use_case
            .execute(HDelInput::new(key, fields))
            .await
    }

    pub async fn hexists(&self, key: Key, field: String) -> Result<bool, DomainError> {
        self.hexists_use_case
            .execute(HExistsInput::new(key, field))
            .await
    }

    pub async fn hkeys(&self, key: Key) -> Result<Vec<String>, DomainError> {
        self.hkeys_use_case.execute(HKeysInput::new(key)).await
    }

    pub async fn hvals(&self, key: Key) -> Result<Vec<Vec<u8>>, DomainError> {
        self.hvals_use_case.execute(HValsInput::new(key)).await
    }

    pub async fn hlen(&self, key: Key) -> Result<usize, DomainError> {
        self.hlen_use_case.execute(HLenInput::new(key)).await
    }
}
