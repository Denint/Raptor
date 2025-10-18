use crate::domain::{
    errors::DomainError,
    repositories::BasicRepository,
    value_objects::{key::Key, value::Value},
};
use metrics::counter;
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SetKeyInput {
    pub key: Key,
    pub value: Value,
}

impl SetKeyInput {
    pub fn new(key: Key, value: Value) -> Self {
        Self { key, value }
    }
}

pub struct SetKeyUseCase<S: ?Sized> {
    repository: Arc<S>,
}

impl<S: ?Sized> SetKeyUseCase<S>
where
    S: BasicRepository + Send + Sync,
{
    pub fn new(repository: Arc<S>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SetKeyInput) -> Result<bool, DomainError> {
        let value_size = input.value.clone().into_vec().map(|v| v.len()).unwrap_or(0);
        let span =
            info_span!("audit.set_use_case", key = %input.key.as_str(), value_size = value_size);
        let _enter = span.enter();

        counter!("raptor.commands_total", "command" => "set").increment(1);
        let result = self.repository.set(&input.key, input.value).await;
        event!(Level::INFO, op = "set", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockBasicRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_set_key_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());
        let input = SetKeyInput::new(key.clone(), value.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.value, value);
    }

    #[tokio::test]
    async fn test_set_key_use_case_new() {
        let mock_repo = Arc::new(MockBasicRepository::new());
        let _use_case = SetKeyUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_set_key_use_case_execute_success() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());

        mock_repo
            .expect_set()
            .with(eq(key.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _| Ok(true));

        let _use_case = SetKeyUseCase::new(Arc::new(mock_repo));
        let input = SetKeyInput::new(key, value);

        let result = _use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_set_key_use_case_execute_error() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let value = Value::Integer(42);

        mock_repo
            .expect_set()
            .with(eq(key.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let _use_case = SetKeyUseCase::new(Arc::new(mock_repo));
        let input = SetKeyInput::new(key, value);

        let result = _use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }

    #[tokio::test]
    async fn test_set_key_use_case_execute_false_result() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("false_key".to_string()).unwrap();
        let value = Value::String(b"test".to_vec());

        mock_repo
            .expect_set()
            .with(eq(key.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _| Ok(false));

        let _use_case = SetKeyUseCase::new(Arc::new(mock_repo));
        let input = SetKeyInput::new(key, value);

        let result = _use_case.execute(input).await.unwrap();
        assert!(!result);
    }
}
