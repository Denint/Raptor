use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::sadd_use_case::SAddUseCase;
use super::operations::scard_use_case::SCardUseCase;
use super::operations::sismember_use_case::SIsMemberUseCase;
use super::operations::smembers_use_case::SMembersUseCase;
use super::operations::srem_use_case::SRemUseCase;

pub struct SetUseCases {
    pub sadd: Arc<SAddUseCase<InMemoryStorage>>,
    pub smembers: Arc<SMembersUseCase<InMemoryStorage>>,
    pub srem: Arc<SRemUseCase<InMemoryStorage>>,
    pub sismember: Arc<SIsMemberUseCase<InMemoryStorage>>,
    pub scard: Arc<SCardUseCase<InMemoryStorage>>,
}

impl SetUseCases {
    pub fn new(
        sadd: Arc<SAddUseCase<InMemoryStorage>>,
        smembers: Arc<SMembersUseCase<InMemoryStorage>>,
        srem: Arc<SRemUseCase<InMemoryStorage>>,
        sismember: Arc<SIsMemberUseCase<InMemoryStorage>>,
        scard: Arc<SCardUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            sadd,
            smembers,
            srem,
            sismember,
            scard,
        }
    }
}
