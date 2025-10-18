use crate::domain::{errors::DomainError, repositories::SetRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SCardInput {
    pub key: Key,
}

impl SCardInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct SCardUseCase<R: ?Sized + SetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SetRepository> SCardUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SCardInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.scard_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.scard(&input.key).await;
        event!(Level::INFO, op = "scard", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_scard_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = SCardInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_scard_use_case_new() {
        let mock_repo = Arc::new(MockSetRepository::new());
        let _use_case = SCardUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_scard_use_case_execute_with_members() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_scard()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(3));

        let use_case = SCardUseCase::new(Arc::new(mock_repo));
        let input = SCardInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_scard_use_case_execute_empty_set() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();

        mock_repo
            .expect_scard()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(0));

        let use_case = SCardUseCase::new(Arc::new(mock_repo));
        let input = SCardInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_scard_use_case_execute_single_member() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();

        mock_repo
            .expect_scard()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(1));

        let use_case = SCardUseCase::new(Arc::new(mock_repo));
        let input = SCardInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_scard_use_case_execute_nonexistent_key() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();

        mock_repo
            .expect_scard()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(0));

        let use_case = SCardUseCase::new(Arc::new(mock_repo));
        let input = SCardInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_scard_use_case_execute_error() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_scard()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = SCardUseCase::new(Arc::new(mock_repo));
        let input = SCardInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
