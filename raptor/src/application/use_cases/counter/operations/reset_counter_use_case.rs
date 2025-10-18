use crate::domain::{
    errors::DomainError, repositories::CounterRepository, value_objects::key::Key,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ResetCounterInput {
    pub key: Key,
}

impl ResetCounterInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct ResetCounterUseCase<S: ?Sized> {
    repository: Arc<S>,
}

impl<S: ?Sized> ResetCounterUseCase<S>
where
    S: CounterRepository + Send + Sync,
{
    pub fn new(repository: Arc<S>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ResetCounterInput) -> Result<Option<i64>, DomainError> {
        let span = info_span!("audit.reset_counter_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.reset(&input.key).await;
        event!(Level::INFO, op = "reset", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockCounterRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_reset_counter_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ResetCounterInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_reset_counter_use_case_new() {
        let mock_repo = Arc::new(MockCounterRepository::new());
        let _use_case = ResetCounterUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_reset_counter_use_case_execute_with_value() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_reset()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(42)));

        let use_case = ResetCounterUseCase::new(Arc::new(mock_repo));
        let input = ResetCounterInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_reset_counter_use_case_execute_no_value() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();

        mock_repo
            .expect_reset()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = ResetCounterUseCase::new(Arc::new(mock_repo));
        let input = ResetCounterInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_reset_counter_use_case_execute_error() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_reset()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ResetCounterUseCase::new(Arc::new(mock_repo));
        let input = ResetCounterInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
