use crate::domain::{errors::DomainError, repositories::SetRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SMembersInput {
    pub key: Key,
}

impl SMembersInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct SMembersUseCase<R: ?Sized + SetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SetRepository> SMembersUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SMembersInput) -> Result<Vec<Vec<u8>>, DomainError> {
        let span = info_span!("audit.smembers_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.smembers(&input.key).await;
        event!(Level::INFO, op = "smembers", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_smembers_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = SMembersInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_smembers_use_case_new() {
        let mock_repo = Arc::new(MockSetRepository::new());
        let _use_case = SMembersUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_smembers_use_case_execute_with_members() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_members = vec![
            b"member1".to_vec(),
            b"member2".to_vec(),
            b"member3".to_vec(),
        ];

        mock_repo
            .expect_smembers()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| {
                Ok(vec![
                    b"member1".to_vec(),
                    b"member2".to_vec(),
                    b"member3".to_vec(),
                ])
            });

        let use_case = SMembersUseCase::new(Arc::new(mock_repo));
        let input = SMembersInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_members);
    }

    #[tokio::test]
    async fn test_smembers_use_case_execute_single_member() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let expected_members = vec![b"single_member".to_vec()];

        mock_repo
            .expect_smembers()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(vec![b"single_member".to_vec()]));

        let use_case = SMembersUseCase::new(Arc::new(mock_repo));
        let input = SMembersInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_members);
    }

    #[tokio::test]
    async fn test_smembers_use_case_execute_empty_set() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let expected_members: Vec<Vec<u8>> = vec![];

        mock_repo
            .expect_smembers()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(vec![]));

        let use_case = SMembersUseCase::new(Arc::new(mock_repo));
        let input = SMembersInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_members);
    }

    #[tokio::test]
    async fn test_smembers_use_case_execute_nonexistent_key() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();
        let expected_members: Vec<Vec<u8>> = vec![];

        mock_repo
            .expect_smembers()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(vec![]));

        let use_case = SMembersUseCase::new(Arc::new(mock_repo));
        let input = SMembersInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_members);
    }

    #[tokio::test]
    async fn test_smembers_use_case_execute_error() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_smembers()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = SMembersUseCase::new(Arc::new(mock_repo));
        let input = SMembersInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
