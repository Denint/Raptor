use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::zadd_use_case::ZAddUseCase;
use super::operations::zcard_use_case::ZCardUseCase;
use super::operations::zrange_use_case::ZRangeUseCase;
use super::operations::zrem_use_case::ZRemUseCase;
use super::operations::zscore_use_case::ZScoreUseCase;

pub struct SortedSetUseCases {
    pub zadd: Arc<ZAddUseCase<InMemoryStorage>>,
    pub zrange: Arc<ZRangeUseCase<InMemoryStorage>>,
    pub zrem: Arc<ZRemUseCase<InMemoryStorage>>,
    pub zscore: Arc<ZScoreUseCase<InMemoryStorage>>,
    pub zcard: Arc<ZCardUseCase<InMemoryStorage>>,
}

impl SortedSetUseCases {
    pub fn new(
        zadd: Arc<ZAddUseCase<InMemoryStorage>>,
        zrange: Arc<ZRangeUseCase<InMemoryStorage>>,
        zrem: Arc<ZRemUseCase<InMemoryStorage>>,
        zscore: Arc<ZScoreUseCase<InMemoryStorage>>,
        zcard: Arc<ZCardUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            zadd,
            zrange,
            zrem,
            zscore,
            zcard,
        }
    }
}
