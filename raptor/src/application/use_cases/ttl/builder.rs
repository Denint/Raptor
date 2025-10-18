use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::expire_use_case::ExpireUseCase;
use super::operations::persist_use_case::PersistUseCase;
use super::operations::set_with_ttl_use_case::SetWithTtlUseCase;
use super::operations::ttl_use_case::TtlUseCase;

pub struct TtlUseCases {
    pub set_with_ttl: Arc<SetWithTtlUseCase<InMemoryStorage>>,
    pub ttl: Arc<TtlUseCase<InMemoryStorage>>,
    pub persist: Arc<PersistUseCase<InMemoryStorage>>,
    pub expire: Arc<ExpireUseCase<InMemoryStorage>>,
}

impl TtlUseCases {
    pub fn new(
        set_with_ttl: Arc<SetWithTtlUseCase<InMemoryStorage>>,
        ttl: Arc<TtlUseCase<InMemoryStorage>>,
        persist: Arc<PersistUseCase<InMemoryStorage>>,
        expire: Arc<ExpireUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            set_with_ttl,
            ttl,
            persist,
            expire,
        }
    }
}
