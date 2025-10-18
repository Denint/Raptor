use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::hdel_use_case::HDelUseCase;
use super::operations::hexists_use_case::HExistsUseCase;
use super::operations::hget_use_case::HGetUseCase;
use super::operations::hkeys_use_case::HKeysUseCase;
use super::operations::hlen_use_case::HLenUseCase;
use super::operations::hset_use_case::HSetUseCase;
use super::operations::hvals_use_case::HValsUseCase;

pub struct HashUseCases {
    pub hset: Arc<HSetUseCase<InMemoryStorage>>,
    pub hget: Arc<HGetUseCase<InMemoryStorage>>,
    pub hdel: Arc<HDelUseCase<InMemoryStorage>>,
    pub hexists: Arc<HExistsUseCase<InMemoryStorage>>,
    pub hkeys: Arc<HKeysUseCase<InMemoryStorage>>,
    pub hvals: Arc<HValsUseCase<InMemoryStorage>>,
    pub hlen: Arc<HLenUseCase<InMemoryStorage>>,
}

impl HashUseCases {
    pub fn new(
        hset: Arc<HSetUseCase<InMemoryStorage>>,
        hget: Arc<HGetUseCase<InMemoryStorage>>,
        hdel: Arc<HDelUseCase<InMemoryStorage>>,
        hexists: Arc<HExistsUseCase<InMemoryStorage>>,
        hkeys: Arc<HKeysUseCase<InMemoryStorage>>,
        hvals: Arc<HValsUseCase<InMemoryStorage>>,
        hlen: Arc<HLenUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            hset,
            hget,
            hdel,
            hexists,
            hkeys,
            hvals,
            hlen,
        }
    }
}
