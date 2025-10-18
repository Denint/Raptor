use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::HashUseCases;
use crate::domain::{errors::DomainError, value_objects::key::Key};

pub struct HashController {
    use_cases: HashUseCases,
}

impl HashController {
    pub fn new(use_cases: HashUseCases) -> Self {
        Self { use_cases }
    }

    pub fn hset_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hset_use_case::HSetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hset
    }

    pub fn hget_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hget_use_case::HGetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hget
    }

    pub fn hdel_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hdel_use_case::HDelUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hdel
    }

    pub fn hexists_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hexists_use_case::HExistsUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hexists
    }

    pub fn hkeys_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hkeys_use_case::HKeysUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hkeys
    }

    pub fn hvals_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hvals_use_case::HValsUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hvals
    }

    pub fn hlen_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::hash::operations::hlen_use_case::HLenUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.hlen
    }

    pub async fn hset(&self, key: Key, field: String, value: Vec<u8>) -> Result<bool, DomainError> {
        self.use_cases
            .hset
            .execute(super::operations::hset_use_case::HSetInput::new(
                key, field, value,
            ))
            .await
    }

    pub async fn hget(&self, key: Key, field: String) -> Result<Option<Vec<u8>>, DomainError> {
        self.use_cases
            .hget
            .execute(super::operations::hget_use_case::HGetInput::new(key, field))
            .await
    }

    pub async fn hdel(&self, key: Key, fields: Vec<String>) -> Result<usize, DomainError> {
        self.use_cases
            .hdel
            .execute(super::operations::hdel_use_case::HDelInput::new(
                key, fields,
            ))
            .await
    }

    pub async fn hexists(&self, key: Key, field: String) -> Result<bool, DomainError> {
        self.use_cases
            .hexists
            .execute(super::operations::hexists_use_case::HExistsInput::new(
                key, field,
            ))
            .await
    }

    pub async fn hkeys(&self, key: Key) -> Result<Vec<String>, DomainError> {
        self.use_cases
            .hkeys
            .execute(super::operations::hkeys_use_case::HKeysInput::new(key))
            .await
    }

    pub async fn hvals(&self, key: Key) -> Result<Vec<Vec<u8>>, DomainError> {
        self.use_cases
            .hvals
            .execute(super::operations::hvals_use_case::HValsInput::new(key))
            .await
    }

    pub async fn hlen(&self, key: Key) -> Result<usize, DomainError> {
        self.use_cases
            .hlen
            .execute(super::operations::hlen_use_case::HLenInput::new(key))
            .await
    }
}
