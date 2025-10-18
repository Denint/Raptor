use crate::domain::{errors::DomainError, repositories::TtlRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct PersistInput {
    pub key: Key,
}

impl PersistInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct PersistUseCase<R: ?Sized + TtlRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + TtlRepository> PersistUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: PersistInput) -> Result<bool, DomainError> {
        let span = info_span!("audit.persist_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.persist(&input.key).await;
        event!(Level::INFO, op = "persist", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_persist_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = PersistInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_persist_use_case_new() {
        let mock_repo = Arc::new(MockTtlRepository::new());
        let _use_case = PersistUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_persist_use_case_execute_success() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_persist()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(true));

        let use_case = PersistUseCase::new(Arc::new(mock_repo));
        let input = PersistInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_persist_use_case_execute_failure() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_persist()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(false));

        let use_case = PersistUseCase::new(Arc::new(mock_repo));
        let input = PersistInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_persist_use_case_execute_error() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_persist()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = PersistUseCase::new(Arc::new(mock_repo));
        let input = PersistInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
