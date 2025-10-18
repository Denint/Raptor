use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::MultiKeyUseCases;
use super::operations::mdel_use_case::MDelInput;
use super::operations::mget_use_case::MGetInput;
use super::operations::mset_use_case::MSetInput;

pub struct MultiKeyController {
    use_cases: MultiKeyUseCases,
}

impl MultiKeyController {
    pub fn new(use_cases: MultiKeyUseCases) -> Self {
        Self { use_cases }
    }

    pub fn mset_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::multi_key::operations::mset_use_case::MSetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.mset
    }

    pub fn mget_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::multi_key::operations::mget_use_case::MGetUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.mget
    }

    pub fn mdel_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::multi_key::operations::mdel_use_case::MDelUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.mdel
    }

    pub async fn mset(
        &self,
        pairs: Vec<(
            crate::domain::value_objects::key::Key,
            crate::domain::value_objects::value::Value,
        )>,
    ) -> Result<(), crate::domain::errors::DomainError> {
        let input = MSetInput::new(pairs);
        self.mset_use_case().execute(input).await
    }

    pub async fn mget(
        &self,
        keys: Vec<crate::domain::value_objects::key::Key>,
    ) -> Result<
        Vec<Option<crate::domain::value_objects::value::Value>>,
        crate::domain::errors::DomainError,
    > {
        let input = MGetInput::new(keys);
        self.mget_use_case().execute(input).await
    }

    pub async fn mdel(
        &self,
        keys: Vec<crate::domain::value_objects::key::Key>,
    ) -> Result<
        Vec<Option<crate::domain::value_objects::value::Value>>,
        crate::domain::errors::DomainError,
    > {
        let input = MDelInput::new(keys);
        self.mdel_use_case().execute(input).await
    }
}
