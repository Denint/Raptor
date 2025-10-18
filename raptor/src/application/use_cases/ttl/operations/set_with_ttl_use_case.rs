use crate::domain::{
    errors::DomainError, repositories::TtlRepository, value_objects::key::Key,
    value_objects::value::Value,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SetWithTtlInput {
    pub key: Key,
    pub value: Value,
    pub ttl_seconds: u64,
}

impl SetWithTtlInput {
    pub fn new(key: Key, value: Value, ttl_seconds: u64) -> Self {
        Self {
            key,
            value,
            ttl_seconds,
        }
    }
}

pub struct SetWithTtlUseCase<R: ?Sized + TtlRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + TtlRepository> SetWithTtlUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SetWithTtlInput) -> Result<(), DomainError> {
        let span = info_span!("audit.set_with_ttl_use_case", key = %input.key.as_str(), ttl_seconds = input.ttl_seconds);
        let _enter = span.enter();
        let result = self
            .repository
            .set_with_ttl(&input.key, input.value, input.ttl_seconds)
            .await;
        event!(Level::INFO, op = "set_with_ttl", key = %input.key.as_str(), success = result.is_ok(), ttl_seconds = input.ttl_seconds);
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
    async fn test_set_with_ttl_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());
        let input = SetWithTtlInput::new(key.clone(), value.clone(), 3600);
        assert_eq!(input.key, key);
        assert_eq!(input.value, value);
        assert_eq!(input.ttl_seconds, 3600);
    }

    #[tokio::test]
    async fn test_set_with_ttl_use_case_new() {
        let mock_repo = Arc::new(MockTtlRepository::new());
        let _use_case = SetWithTtlUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_set_with_ttl_use_case_execute_success() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let value = Value::String(b"test_value".to_vec());

        mock_repo
            .expect_set_with_ttl()
            .with(eq(key.clone()), eq(value.clone()), eq(3600u64))
            .times(1)
            .returning(|_, _, _| Ok(()));

        let use_case = SetWithTtlUseCase::new(Arc::new(mock_repo));
        let input = SetWithTtlInput::new(key, value, 3600);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_with_ttl_use_case_execute_error() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let value = Value::Integer(42);

        mock_repo
            .expect_set_with_ttl()
            .with(eq(key.clone()), eq(value.clone()), eq(3600u64))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = SetWithTtlUseCase::new(Arc::new(mock_repo));
        let input = SetWithTtlInput::new(key, value, 3600);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
