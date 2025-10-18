use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::array_append_use_case::ArrayAppendUseCase;
use super::operations::array_get_use_case::ArrayGetUseCase;
use super::operations::array_length_use_case::ArrayLengthUseCase;
use super::operations::array_set_use_case::ArraySetUseCase;
use super::operations::array_slice_use_case::ArraySliceUseCase;
use super::operations::array_update_use_case::ArrayUpdateUseCase;

pub struct ArrayUseCases {
    pub array_set: Arc<ArraySetUseCase<InMemoryStorage>>,
    pub array_get: Arc<ArrayGetUseCase<InMemoryStorage>>,
    pub array_append: Arc<ArrayAppendUseCase<InMemoryStorage>>,
    pub array_slice: Arc<ArraySliceUseCase<InMemoryStorage>>,
    pub array_update: Arc<ArrayUpdateUseCase<InMemoryStorage>>,
    pub array_length: Arc<ArrayLengthUseCase<InMemoryStorage>>,
}

impl ArrayUseCases {
    pub fn new(
        array_set: Arc<ArraySetUseCase<InMemoryStorage>>,
        array_get: Arc<ArrayGetUseCase<InMemoryStorage>>,
        array_append: Arc<ArrayAppendUseCase<InMemoryStorage>>,
        array_slice: Arc<ArraySliceUseCase<InMemoryStorage>>,
        array_update: Arc<ArrayUpdateUseCase<InMemoryStorage>>,
        array_length: Arc<ArrayLengthUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            array_set,
            array_get,
            array_append,
            array_slice,
            array_update,
            array_length,
        }
    }
}
