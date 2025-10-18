use crate::{
    application::use_cases::list::operations::{
        lpop_use_case::{LPopInput, LPopUseCase},
        lpush_use_case::{LPushInput, LPushUseCase},
        lrange_use_case::{LRangeInput, LRangeUseCase},
        rpop_use_case::{RPopInput, RPopUseCase},
        rpush_use_case::{RPushInput, RPushUseCase},
    },
    domain::{errors::DomainError, repositories::ListRepository, value_objects::key::Key},
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

pub struct ListController {
    lpush_use_case: Arc<LPushUseCase<InMemoryStorage>>,
    rpush_use_case: Arc<RPushUseCase<InMemoryStorage>>,
    lpop_use_case: Arc<LPopUseCase<InMemoryStorage>>,
    rpop_use_case: Arc<RPopUseCase<InMemoryStorage>>,
    lrange_use_case: Arc<LRangeUseCase<InMemoryStorage>>,
}

impl ListController {
    pub fn new(
        lpush_use_case: Arc<LPushUseCase<InMemoryStorage>>,
        rpush_use_case: Arc<RPushUseCase<InMemoryStorage>>,
        lpop_use_case: Arc<LPopUseCase<InMemoryStorage>>,
        rpop_use_case: Arc<RPopUseCase<InMemoryStorage>>,
        lrange_use_case: Arc<LRangeUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            lpush_use_case,
            rpush_use_case,
            lpop_use_case,
            rpop_use_case,
            lrange_use_case,
        }
    }

    pub async fn lpush(&self, key: Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        self.lpush_use_case
            .execute(LPushInput::new(key, values))
            .await
    }

    pub async fn rpush(&self, key: Key, values: Vec<Vec<u8>>) -> Result<usize, DomainError> {
        self.rpush_use_case
            .execute(RPushInput::new(key, values))
            .await
    }

    pub async fn lpop(&self, key: Key) -> Result<Option<Vec<u8>>, DomainError> {
        self.lpop_use_case.execute(LPopInput::new(key)).await
    }

    pub async fn rpop(&self, key: Key) -> Result<Option<Vec<u8>>, DomainError> {
        self.rpop_use_case.execute(RPopInput::new(key)).await
    }

    pub async fn lrange(
        &self,
        key: Key,
        start: isize,
        stop: isize,
    ) -> Result<Vec<Vec<u8>>, DomainError> {
        self.lrange_use_case
            .execute(LRangeInput::new(key, start, stop))
            .await
    }
}
