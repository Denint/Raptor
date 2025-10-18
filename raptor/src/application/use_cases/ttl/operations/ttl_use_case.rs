use crate::domain::{errors::DomainError, repositories::TtlRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct TtlInput {
    pub key: Key,
}

impl TtlInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct TtlUseCase<R: ?Sized + TtlRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + TtlRepository> TtlUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: TtlInput) -> Result<Option<i64>, DomainError> {
        let span = info_span!("audit.ttl_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.ttl(&input.key).await;
        event!(Level::INFO, op = "ttl", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockTtlRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_ttl_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = TtlInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_ttl_use_case_new() {
        let mock_repo = Arc::new(MockTtlRepository::new());
        let _use_case = TtlUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_ttl_use_case_execute_with_ttl() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_ttl()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(3600)));

        let use_case = TtlUseCase::new(Arc::new(mock_repo));
        let input = TtlInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(3600));
    }

    #[tokio::test]
    async fn test_ttl_use_case_execute_no_ttl() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("no_ttl_key".to_string()).unwrap();

        mock_repo
            .expect_ttl()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = TtlUseCase::new(Arc::new(mock_repo));
        let input = TtlInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_ttl_use_case_execute_error() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_ttl()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = TtlUseCase::new(Arc::new(mock_repo));
        let input = TtlInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
