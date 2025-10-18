use crate::application::use_cases::array::operations::array_append_use_case::{
    ArrayAppendInput, ArrayAppendUseCase,
};
use crate::application::use_cases::array::operations::array_get_use_case::{ArrayGetInput, ArrayGetUseCase};
use crate::application::use_cases::array::operations::array_length_use_case::{
    ArrayLengthInput, ArrayLengthUseCase,
};
use crate::application::use_cases::array::operations::array_set_use_case::{ArraySetInput, ArraySetUseCase};
use crate::application::use_cases::array::operations::array_slice_use_case::{
    ArraySliceInput, ArraySliceUseCase,
};
use crate::application::use_cases::array::operations::array_update_use_case::{
    ArrayUpdateInput, ArrayUpdateUseCase,
};
use crate::application::use_cases::basic::operations::delete_key_use_case::{DeleteKeyInput, DeleteKeyUseCase};
use crate::application::use_cases::basic::operations::get_key_use_case::{GetKeyInput, GetKeyUseCase};
use crate::application::use_cases::basic::operations::set_key_use_case::{SetKeyInput, SetKeyUseCase};
use crate::application::use_cases::ttl::operations::expire_use_case::{ExpireInput, ExpireUseCase};
use crate::application::use_cases::ttl::operations::persist_use_case::{PersistInput, PersistUseCase};
use crate::application::use_cases::ttl::operations::set_with_ttl_use_case::{
    SetWithTtlInput, SetWithTtlUseCase,
};
use crate::application::use_cases::ttl::operations::ttl_use_case::{TtlInput, TtlUseCase};
use crate::infrastructure::persistence::InMemoryStorage;
use crate::domain::{
    errors::DomainError,
    repositories::{ArrayRepository, BasicRepository, TtlRepository},
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;

pub struct SingleKeyController {
    get_key_use_case: Arc<GetKeyUseCase<InMemoryStorage>>,
    set_key_use_case: Arc<SetKeyUseCase<InMemoryStorage>>,
    delete_key_use_case: Arc<DeleteKeyUseCase<InMemoryStorage>>,
    set_with_ttl_use_case: Arc<SetWithTtlUseCase<InMemoryStorage>>,
    ttl_use_case: Arc<TtlUseCase<InMemoryStorage>>,
    persist_use_case: Arc<PersistUseCase<InMemoryStorage>>,
    expire_use_case: Arc<ExpireUseCase<InMemoryStorage>>,
    array_set_use_case: Arc<ArraySetUseCase<InMemoryStorage>>,
    array_get_use_case: Arc<ArrayGetUseCase<InMemoryStorage>>,
    array_append_use_case: Arc<ArrayAppendUseCase<InMemoryStorage>>,
    array_slice_use_case: Arc<ArraySliceUseCase<InMemoryStorage>>,
    array_update_use_case: Arc<ArrayUpdateUseCase<InMemoryStorage>>,
    array_length_use_case: Arc<ArrayLengthUseCase<InMemoryStorage>>,
}

impl SingleKeyController {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        get_key_use_case: Arc<GetKeyUseCase<B>>,
        set_key_use_case: Arc<SetKeyUseCase<B>>,
        delete_key_use_case: Arc<DeleteKeyUseCase<B>>,
        set_with_ttl_use_case: Arc<SetWithTtlUseCase<T>>,
        ttl_use_case: Arc<TtlUseCase<T>>,
        persist_use_case: Arc<PersistUseCase<T>>,
        expire_use_case: Arc<ExpireUseCase<T>>,
        array_set_use_case: Arc<ArraySetUseCase<A>>,
        array_get_use_case: Arc<ArrayGetUseCase<A>>,
        array_append_use_case: Arc<ArrayAppendUseCase<A>>,
        array_slice_use_case: Arc<ArraySliceUseCase<A>>,
        array_update_use_case: Arc<ArrayUpdateUseCase<A>>,
        array_length_use_case: Arc<ArrayLengthUseCase<A>>,
    ) -> Self {
        Self {
            get_key_use_case,
            set_key_use_case,
            delete_key_use_case,
            set_with_ttl_use_case,
            ttl_use_case,
            persist_use_case,
            expire_use_case,
            array_set_use_case,
            array_get_use_case,
            array_append_use_case,
            array_slice_use_case,
            array_update_use_case,
            array_length_use_case,
        }
    }

    pub async fn get_key(&self, key: Key) -> Result<Option<Value>, DomainError> {
        self.get_key_use_case.execute(GetKeyInput::new(key)).await
    }

    pub async fn set_key(&self, key: Key, value: Value) -> Result<bool, DomainError> {
        self.set_key_use_case
            .execute(SetKeyInput::new(key, value))
            .await
    }

    pub async fn delete_key(&self, key: Key) -> Result<Option<Value>, DomainError> {
        self.delete_key_use_case
            .execute(DeleteKeyInput::new(key))
            .await
    }

    pub async fn set_key_with_ttl(
        &self,
        key: Key,
        value: Value,
        ttl_seconds: u64,
    ) -> Result<(), DomainError> {
        self.set_with_ttl_use_case
            .execute(SetWithTtlInput::new(key, value, ttl_seconds))
            .await
    }

    pub async fn get_ttl(&self, key: Key) -> Result<Option<i64>, DomainError> {
        self.ttl_use_case.execute(TtlInput::new(key)).await
    }

    pub async fn persist_key(&self, key: Key) -> Result<bool, DomainError> {
        self.persist_use_case.execute(PersistInput::new(key)).await
    }

    pub async fn expire_key(&self, key: Key, ttl_seconds: u64) -> Result<bool, DomainError> {
        self.expire_use_case
            .execute(ExpireInput::new(key, ttl_seconds))
            .await
    }

    pub async fn array_set(&self, key: Key, values: Vec<Value>) -> Result<(), DomainError> {
        self.array_set_use_case
            .execute(ArraySetInput::new(key, values))
            .await
    }

    pub async fn array_get(
        &self,
        key: Key,
        indices: Vec<usize>,
    ) -> Result<Vec<Option<Value>>, DomainError> {
        self.array_get_use_case
            .execute(ArrayGetInput::new(key, indices))
            .await
    }

    pub async fn array_append(&self, key: Key, values: Vec<Value>) -> Result<usize, DomainError> {
        self.array_append_use_case
            .execute(ArrayAppendInput::new(key, values))
            .await
    }

    pub async fn array_slice(
        &self,
        key: Key,
        start: usize,
        end: Option<usize>,
    ) -> Result<Vec<Value>, DomainError> {
        self.array_slice_use_case
            .execute(ArraySliceInput::new(key, start, end))
            .await
    }

    pub async fn array_update(
        &self,
        key: Key,
        updates: Vec<(usize, Value)>,
    ) -> Result<usize, DomainError> {
        self.array_update_use_case
            .execute(ArrayUpdateInput::new(key, updates))
            .await
    }

    pub async fn array_length(&self, key: Key) -> Result<Option<usize>, DomainError> {
        self.array_length_use_case
            .execute(ArrayLengthInput::new(key))
            .await
    }
}
