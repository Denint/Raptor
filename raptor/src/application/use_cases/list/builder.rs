use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::lpop_use_case::LPopUseCase;
use super::operations::lpush_use_case::LPushUseCase;
use super::operations::lrange_use_case::LRangeUseCase;
use super::operations::rpop_use_case::RPopUseCase;
use super::operations::rpush_use_case::RPushUseCase;

pub struct ListUseCases {
    pub lpush: Arc<LPushUseCase<InMemoryStorage>>,
    pub rpush: Arc<RPushUseCase<InMemoryStorage>>,
    pub lpop: Arc<LPopUseCase<InMemoryStorage>>,
    pub rpop: Arc<RPopUseCase<InMemoryStorage>>,
    pub lrange: Arc<LRangeUseCase<InMemoryStorage>>,
}

impl ListUseCases {
    pub fn new(
        lpush: Arc<LPushUseCase<InMemoryStorage>>,
        rpush: Arc<RPushUseCase<InMemoryStorage>>,
        lpop: Arc<LPopUseCase<InMemoryStorage>>,
        rpop: Arc<RPopUseCase<InMemoryStorage>>,
        lrange: Arc<LRangeUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            lpush,
            rpush,
            lpop,
            rpop,
            lrange,
        }
    }
}
