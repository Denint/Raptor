use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::ListUseCases;
use super::operations::lpop_use_case::LPopInput;
use super::operations::lpush_use_case::LPushInput;
use super::operations::lrange_use_case::LRangeInput;
use super::operations::rpop_use_case::RPopInput;
use super::operations::rpush_use_case::RPushInput;

pub struct ListController {
    use_cases: ListUseCases,
}

impl ListController {
    pub fn new(use_cases: ListUseCases) -> Self {
        Self { use_cases }
    }

    pub fn lpush_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::list::operations::lpush_use_case::LPushUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.lpush
    }

    pub fn rpush_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::list::operations::rpush_use_case::RPushUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.rpush
    }

    pub fn lpop_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::list::operations::lpop_use_case::LPopUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.lpop
    }

    pub fn rpop_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::list::operations::rpop_use_case::RPopUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.rpop
    }

    pub fn lrange_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::list::operations::lrange_use_case::LRangeUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.lrange
    }

    pub async fn lpush(
        &self,
        key: crate::domain::value_objects::key::Key,
        values: Vec<Vec<u8>>,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = LPushInput::new(key, values);
        self.lpush_use_case().execute(input).await
    }

    pub async fn rpush(
        &self,
        key: crate::domain::value_objects::key::Key,
        values: Vec<Vec<u8>>,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = RPushInput::new(key, values);
        self.rpush_use_case().execute(input).await
    }

    pub async fn lpop(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<Option<Vec<u8>>, crate::domain::errors::DomainError> {
        let input = LPopInput::new(key);
        self.lpop_use_case().execute(input).await
    }

    pub async fn rpop(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<Option<Vec<u8>>, crate::domain::errors::DomainError> {
        let input = RPopInput::new(key);
        self.rpop_use_case().execute(input).await
    }

    pub async fn lrange(
        &self,
        key: crate::domain::value_objects::key::Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, crate::domain::errors::DomainError> {
        let input = LRangeInput::new(key, start, stop);
        self.lrange_use_case().execute(input).await
    }
}
