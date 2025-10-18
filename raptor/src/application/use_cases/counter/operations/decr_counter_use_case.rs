use crate::domain::{
    errors::DomainError, repositories::CounterRepository, value_objects::key::Key,
};
use metrics::counter;
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct DecrCounterInput {
    pub key: Key,
}

impl DecrCounterInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct DecrCounterUseCase<S: ?Sized> {
    repository: Arc<S>,
}

impl<S: ?Sized> DecrCounterUseCase<S>
where
    S: CounterRepository + Send + Sync,
{
    pub fn new(repository: Arc<S>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: DecrCounterInput) -> Result<i64, DomainError> {
        let span = info_span!("audit.decr_counter_use_case", key = %input.key.as_str());
        let _enter = span.enter();

        counter!("raptor.commands_total", "command" => "decr").increment(1);

        let result = self.repository.decr(&input.key).await;
        event!(Level::INFO, op = "decr", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_decr_counter_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = DecrCounterInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_decr_counter_use_case_new() {
        let mock_repo = Arc::new(MockCounterRepository::new());
        let _use_case = DecrCounterUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_decr_counter_use_case_execute_success() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_decr()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(4));

        let use_case = DecrCounterUseCase::new(Arc::new(mock_repo));
        let input = DecrCounterInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_decr_counter_use_case_execute_negative_prevention() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("negative_key".to_string()).unwrap();

        mock_repo
            .expect_decr()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::NegativeCounterValue));

        let use_case = DecrCounterUseCase::new(Arc::new(mock_repo));
        let input = DecrCounterInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::NegativeCounterValue)));
    }

    #[tokio::test]
    async fn test_decr_counter_use_case_execute_negative_current_value() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("negative_key".to_string()).unwrap();

        mock_repo
            .expect_decr()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::NegativeCounterValue));

        let use_case = DecrCounterUseCase::new(Arc::new(mock_repo));
        let input = DecrCounterInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::NegativeCounterValue)));
    }

    #[tokio::test]
    async fn test_decr_counter_use_case_execute_storage_error() {
        let mut mock_repo = MockCounterRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_decr()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("decrement error".to_string())));

        let use_case = DecrCounterUseCase::new(Arc::new(mock_repo));
        let input = DecrCounterInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
