use std::sync::Arc;

use crate::infrastructure::persistence::InMemoryStorage;

use super::builder::SetUseCases;
use super::operations::sadd_use_case::SAddInput;
use super::operations::scard_use_case::SCardInput;
use super::operations::sismember_use_case::SIsMemberInput;
use super::operations::smembers_use_case::SMembersInput;
use super::operations::srem_use_case::SRemInput;

pub struct SetController {
    use_cases: SetUseCases,
}

impl SetController {
    pub fn new(use_cases: SetUseCases) -> Self {
        Self { use_cases }
    }

    pub fn sadd_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::set::operations::sadd_use_case::SAddUseCase<InMemoryStorage>,
    > {
        &self.use_cases.sadd
    }

    pub fn smembers_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::set::operations::smembers_use_case::SMembersUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.smembers
    }

    pub fn srem_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::set::operations::srem_use_case::SRemUseCase<InMemoryStorage>,
    > {
        &self.use_cases.srem
    }

    pub fn sismember_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::set::operations::sismember_use_case::SIsMemberUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.sismember
    }

    pub fn scard_use_case(
        &self,
    ) -> &Arc<
        crate::application::use_cases::set::operations::scard_use_case::SCardUseCase<
            InMemoryStorage,
        >,
    > {
        &self.use_cases.scard
    }

    pub async fn sadd(
        &self,
        key: crate::domain::value_objects::key::Key,
        members: Vec<Vec<u8>>,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = SAddInput::new(key, members);
        self.sadd_use_case().execute(input).await
    }

    pub async fn smembers(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<Vec<Vec<u8>>, crate::domain::errors::DomainError> {
        let input = SMembersInput::new(key);
        self.smembers_use_case().execute(input).await
    }

    pub async fn srem(
        &self,
        key: crate::domain::value_objects::key::Key,
        members: Vec<Vec<u8>>,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = SRemInput::new(key, members);
        self.srem_use_case().execute(input).await
    }

    pub async fn sismember(
        &self,
        key: crate::domain::value_objects::key::Key,
        member: &[u8],
    ) -> Result<bool, crate::domain::errors::DomainError> {
        let input = SIsMemberInput::new(key, member.to_vec());
        self.sismember_use_case().execute(input).await
    }

    pub async fn scard(
        &self,
        key: crate::domain::value_objects::key::Key,
    ) -> Result<usize, crate::domain::errors::DomainError> {
        let input = SCardInput::new(key);
        self.scard_use_case().execute(input).await
    }
}
