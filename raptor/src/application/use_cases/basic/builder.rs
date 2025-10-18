use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::operations::delete_key_use_case::DeleteKeyUseCase;
use super::operations::get_key_use_case::GetKeyUseCase;
use super::operations::set_key_use_case::SetKeyUseCase;

pub struct BasicUseCases {
    pub get_key: Arc<GetKeyUseCase<InMemoryStorage>>,
    pub set_key: Arc<SetKeyUseCase<InMemoryStorage>>,
    pub delete_key: Arc<DeleteKeyUseCase<InMemoryStorage>>,
}

impl BasicUseCases {
    pub fn new(
        get_key: Arc<GetKeyUseCase<InMemoryStorage>>,
        set_key: Arc<SetKeyUseCase<InMemoryStorage>>,
        delete_key: Arc<DeleteKeyUseCase<InMemoryStorage>>,
    ) -> Self {
        Self {
            get_key,
            set_key,
            delete_key,
        }
    }
}
