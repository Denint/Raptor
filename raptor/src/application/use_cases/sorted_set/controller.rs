use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::SortedSetUseCases;
use super::operations::zadd_use_case::ZAddInput;
use super::operations::zcard_use_case::ZCardInput;
use super::operations::zrange_use_case::ZRangeInput;
use super::operations::zrem_use_case::ZRemInput;
use super::operations::zscore_use_case::ZScoreInput;

pub struct SortedSetController {
    use_cases: SortedSetUseCases,
}

impl SortedSetController {
    pub fn new(use_cases: SortedSetUseCases) -> Self {
        Self { use_cases }
    }

    pub fn zadd_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::sorted_set::operations::zadd_use_case::ZAddUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.zadd
    }

    pub fn zrange_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::sorted_set::operations::zrange_use_case::ZRangeUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.zrange
    }

    pub fn zrem_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::sorted_set::operations::zrem_use_case::ZRemUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.zrem
    }

    pub fn zscore_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::sorted_set::operations::zscore_use_case::ZScoreUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.zscore
    }

    pub fn zcard_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::sorted_set::operations::zcard_use_case::ZCardUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.zcard
    }

    pub async fn zadd(
        &self,
        key: crate::domain::value_objects::key::Key,
        score: f64,
        member: Vec<u8>,
    ) -> Result<bool, crate::domain::errors::DomainError> {
        let input = ZAddInput::new(key, score, member);
        self.zadd_use_case().execute(input).await
    }

    pub async fn zrange(
        &self,
        key: crate::domain::value_objects::key::Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, crate::domain::errors::DomainError> {
        let input = ZRangeInput::new(key, start, stop);
        self.zrange_use_case().execute(input).await
    }

    pub async fn zrem(
        &self,
        key: crate::domain::value_objects::key::Key,
        members: Vec<Vec<u8>>,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = ZRemInput::new(key, members);
        self.zrem_use_case().execute(input).await
    }

    pub async fn zscore(
        &self,
        key: crate::domain::value_objects::key::Key,
        member: &[u8],
    ) -> Result<Option<f64>, crate::domain::errors::DomainError> {
        let input = ZScoreInput::new(key, member.to_vec());
        self.zscore_use_case().execute(input).await
    }

    pub async fn zcard(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = ZCardInput::new(key);
        self.zcard_use_case().execute(input).await
    }
}
