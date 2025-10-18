use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::BasicUseCases;
use crate::domain::{errors::DomainError, value_objects::key::Key};

pub struct BasicController {
    use_cases: BasicUseCases,
}

impl BasicController {
    pub fn new(use_cases: BasicUseCases) -> Self {
        Self { use_cases }
    }

    pub fn get_key_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::basic::operations::get_key_use_case::GetKeyUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.get_key
    }

    pub fn set_key_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::basic::operations::set_key_use_case::SetKeyUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.set_key
    }

    pub fn delete_key_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::basic::operations::delete_key_use_case::DeleteKeyUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.delete_key
    }

    pub async fn get_key(
        &self,
        key: Key,
    ) -> Result<Option<crate::domain::value_objects::value::Value>, DomainError> {
        self.use_cases
            .get_key
            .execute(super::operations::get_key_use_case::GetKeyInput::new(key))
            .await
    }

    pub async fn set_key(
        &self,
        key: Key,
        value: crate::domain::value_objects::value::Value,
    ) -> Result<bool, DomainError> {
        self.use_cases
            .set_key
            .execute(super::operations::set_key_use_case::SetKeyInput::new(
                key, value,
            ))
            .await
    }

    pub async fn delete_key(
        &self,
        key: Key,
    ) -> Result<Option<crate::domain::value_objects::value::Value>, DomainError> {
        self.use_cases
            .delete_key
            .execute(super::operations::delete_key_use_case::DeleteKeyInput::new(
                key,
            ))
            .await
    }
}
