use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::mdel_use_case::MDelUseCase;
use super::operations::mget_use_case::MGetUseCase;
use super::operations::mset_use_case::MSetUseCase;

pub struct MultiKeyUseCases {
    pub mset: Arc<MSetUseCase<InMemoryStorage>>,
    pub mget: Arc<MGetUseCase<InMemoryStorage>>,
    pub mdel: Arc<MDelUseCase<InMemoryStorage>>,
}

impl MultiKeyUseCases {
    pub fn new(
        mset: Arc<MSetUseCase<InMemoryStorage>>,
        mget: Arc<MGetUseCase<InMemoryStorage>>,
        mdel: Arc<MDelUseCase<InMemoryStorage>>,
    ) -> Self {
        Self { mset, mget, mdel }
    }
}
