use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::ArrayUseCases;
use crate::domain::{errors::DomainError, value_objects::key::Key};

pub struct ArrayController {
    use_cases: ArrayUseCases,
}

impl ArrayController {
    pub fn new(use_cases: ArrayUseCases) -> Self {
        Self { use_cases }
    }

    pub fn array_set_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_set_use_case::ArraySetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_set
    }

    pub fn array_get_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_get_use_case::ArrayGetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_get
    }

    pub fn array_append_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_append_use_case::ArrayAppendUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_append
    }

    pub fn array_slice_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_slice_use_case::ArraySliceUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_slice
    }

    pub fn array_update_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_update_use_case::ArrayUpdateUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_update
    }

    pub fn array_length_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::array::operations::array_length_use_case::ArrayLengthUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.array_length
    }

    pub async fn array_set(
        &self,
        key: Key,
        values: Vec<crate::domain::value_objects::value::Value>,
    ) -> Result<(), DomainError> {
        self.use_cases
            .array_set
            .execute(super::operations::array_set_use_case::ArraySetInput::new(
                key, values,
            ))
            .await
    }

    pub async fn array_get(
        &self,
        key: Key,
        indices: Vec<usize>,
    ) -> Result<Vec<Option<crate::domain::value_objects::value::Value>>, DomainError> {
        self.use_cases
            .array_get
            .execute(super::operations::array_get_use_case::ArrayGetInput::new(
                key, indices,
            ))
            .await
    }

    pub async fn array_append(
        &self,
        key: Key,
        values: Vec<crate::domain::value_objects::value::Value>,
    ) -> Result<usize, DomainError> {
        self.use_cases
            .array_append
            .execute(super::operations::array_append_use_case::ArrayAppendInput::new(key, values))
            .await
    }

    pub async fn array_slice(
        &self,
        key: Key,
        start: usize,
        end: Option<usize>,
    ) -> Result<Vec<crate::domain::value_objects::value::Value>, DomainError> {
        self.use_cases
            .array_slice
            .execute(super::operations::array_slice_use_case::ArraySliceInput::new(key, start, end))
            .await
    }

    pub async fn array_update(
        &self,
        key: Key,
        updates: Vec<(usize, crate::domain::value_objects::value::Value)>,
    ) -> Result<usize, DomainError> {
        self.use_cases
            .array_update
            .execute(super::operations::array_update_use_case::ArrayUpdateInput::new(key, updates))
            .await
    }

    pub async fn array_length(&self, key: Key) -> Result<Option<usize>, DomainError> {
        self.use_cases
            .array_length
            .execute(super::operations::array_length_use_case::ArrayLengthInput::new(key))
            .await
    }
}
