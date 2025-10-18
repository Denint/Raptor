use crate::domain::{
    errors::DomainError, repositories::CounterRepository, value_objects::key::Key,
};
use metrics::counter;
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct IncrCounterInput {
    pub key: Key,
}

impl IncrCounterInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct IncrCounterUseCase<S: ?Sized> {
    repository: Arc<S>,
}

impl<S: ?Sized> IncrCounterUseCase<S>
where
    S: CounterRepository + Send + Sync,
{
    pub fn new(repository: Arc<S>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: IncrCounterInput) -> Result<i64, DomainError> {
        let span = info_span!("audit.incr_counter_use_case", key = %input.key.as_str());
        let _enter = span.enter();

        counter!("raptor.commands_total", "command" => "incr").increment(1);
        let result = self.repository.incr(&input.key, 1).await;
        event!(Level::INFO, op = "incr", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_incr_counter_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = IncrCounterInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_incr_counter_use_case_new() {
        let mock_repo = Arc::new(MockCounterRepository::new());
        let _use_case = IncrCounterUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_incr_counter_use_case_execute_success() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_incr()
            .with(eq(key.clone()), eq(1i64))
            .times(1)
            .returning(|_, _| Ok(5));

        let use_case = IncrCounterUseCase::new(Arc::new(mock_repo));
        let input = IncrCounterInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 5);
    }

    #[tokio::test]
    async fn test_incr_counter_use_case_execute_first_increment() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("new_key".to_string()).unwrap();

        mock_repo
            .expect_incr()
            .with(eq(key.clone()), eq(1i64))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = IncrCounterUseCase::new(Arc::new(mock_repo));
        let input = IncrCounterInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_incr_counter_use_case_execute_error() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_incr()
            .with(eq(key.clone()), eq(1i64))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = IncrCounterUseCase::new(Arc::new(mock_repo));
        let input = IncrCounterInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
