use crate::{
    application::use_cases::multi_key::operations::{
        mdel_use_case::{MDelInput, MDelUseCase},
        mget_use_case::{MGetInput, MGetUseCase},
        mset_use_case::{MSetInput, MSetUseCase},
    },
    domain::{
        errors::DomainError,
        repositories::MultiRepository,
        value_objects::{key::Key, value::Value},
    },
    infrastructure::persistence::InMemoryStorage,
};
use std::sync::Arc;

pub struct MultiKeyController {
    mset_use_case: Arc<MSetUseCase<InMemoryStorage>>,
    mget_use_case: Arc<MGetUseCase<InMemoryStorage>>,
    mdel_use_case: Arc<MDelUseCase<InMemoryStorage>>,
}

impl MultiKeyController {
    pub fn new(
        mset_use_case: Arc<MSetUseCase<InMemoryStorage>>,
        mget_use_case: Arc<MGetUseCase<InMemoryStorage>>,
        mdel_use_case: Arc<MDelUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            mset_use_case,
            mget_use_case,
            mdel_use_case,
        }
    }

    pub async fn mset(&self, pairs: Vec<(Key, Value)>) -> Result<(), DomainError> {
        self.mset_use_case.execute(MSetInput::new(pairs)).await
    }

    pub async fn mget(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError> {
        self.mget_use_case.execute(MGetInput::new(keys)).await
    }

    pub async fn mdel(&self, keys: Vec<Key>) -> Result<Vec<Option<Value>>, DomainError> {
        self.mdel_use_case.execute(MDelInput::new(keys)).await
    }
}
