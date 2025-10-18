use crate::domain::{errors::DomainError, repositories::SetRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SIsMemberInput {
    pub key: Key,
    pub member: Vec<u8>,
}

impl SIsMemberInput {
    pub fn new(key: Key, member: Vec<u8>) -> Self {
        Self { key, member }
    }
}

pub struct SIsMemberUseCase<R: ?Sized + SetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SetRepository> SIsMemberUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SIsMemberInput) -> Result<bool, DomainError> {
        let span = info_span!("audit.sismember_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.sismember(&input.key, &input.member).await;
        event!(Level::INFO, op = "sismember", key = %input.key.as_str(), member = %String::from_utf8_lossy(&input.member), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockSetRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_sismember_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"test_member".to_vec();
        let input = SIsMemberInput::new(key.clone(), member.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.member, member);
    }

    #[tokio::test]
    async fn test_sismember_use_case_new() {
        let mock_repo = Arc::new(MockSetRepository::new());
        let _use_case = SIsMemberUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_sismember_use_case_execute_member_exists() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"existing_member".to_vec();

        mock_repo
            .expect_sismember()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(true));

        let use_case = SIsMemberUseCase::new(Arc::new(mock_repo));
        let input = SIsMemberInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_sismember_use_case_execute_member_not_exists() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"nonexistent_member".to_vec();

        mock_repo
            .expect_sismember()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(false));

        let use_case = SIsMemberUseCase::new(Arc::new(mock_repo));
        let input = SIsMemberInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_sismember_use_case_execute_empty_member() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = vec![];

        mock_repo
            .expect_sismember()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(false));

        let use_case = SIsMemberUseCase::new(Arc::new(mock_repo));
        let input = SIsMemberInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_sismember_use_case_execute_key_not_exists() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();
        let member = b"member".to_vec();

        mock_repo
            .expect_sismember()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(false));

        let use_case = SIsMemberUseCase::new(Arc::new(mock_repo));
        let input = SIsMemberInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_sismember_use_case_execute_error() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let member = b"error_member".to_vec();

        mock_repo
            .expect_sismember()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = SIsMemberUseCase::new(Arc::new(mock_repo));
        let input = SIsMemberInput::new(key, member);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
