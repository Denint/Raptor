use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::TtlUseCases;
use crate::domain::{errors::DomainError, value_objects::key::Key};

pub struct TtlController {
    use_cases: TtlUseCases,
}

impl TtlController {
    pub fn new(use_cases: TtlUseCases) -> Self {
        Self { use_cases }
    }

    pub fn set_with_ttl_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::ttl::operations::set_with_ttl_use_case::SetWithTtlUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.set_with_ttl
    }

    pub fn ttl_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::ttl::operations::ttl_use_case::TtlUseCase<InMemoryStorage>,
    > {
        &self.use_cases.ttl
    }

    pub fn persist_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::ttl::operations::persist_use_case::PersistUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.persist
    }

    pub fn expire_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::ttl::operations::expire_use_case::ExpireUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.expire
    }

    pub async fn set_with_ttl(
        &self,
        key: Key,
        value: crate::domain::value_objects::value::Value,
        ttl_seconds: u64,
    ) -> Result<(), DomainError> {
        self.use_cases
            .set_with_ttl
            .execute(
                super::operations::set_with_ttl_use_case::SetWithTtlInput::new(
                    key,
                    value,
                    ttl_seconds,
                ),
            )
            .await
    }

    pub async fn ttl(&self, key: Key) -> Result<Option<i64>, DomainError> {
        self.use_cases
            .ttl
            .execute(super::operations::ttl_use_case::TtlInput::new(key))
            .await
    }

    pub async fn persist(&self, key: Key) -> Result<bool, DomainError> {
        self.use_cases
            .persist
            .execute(super::operations::persist_use_case::PersistInput::new(key))
            .await
    }

    pub async fn expire(&self, key: Key, ttl_seconds: u64) -> Result<bool, DomainError> {
        self.use_cases
            .expire
            .execute(super::operations::expire_use_case::ExpireInput::new(
                key,
                ttl_seconds,
            ))
            .await
    }
}
