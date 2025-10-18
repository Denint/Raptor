use crate::domain::{
    errors::DomainError,
    repositories::BasicRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct GetKeyInput {
    pub key: Key,
}

impl GetKeyInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct GetKeyUseCase<S: ?Sized> {
    repository: Arc<S>,
}

impl<S: ?Sized> GetKeyUseCase<S>
where
    S: BasicRepository + Send + Sync,
{
    pub fn new(repository: Arc<S>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: GetKeyInput) -> Result<Option<Value>, DomainError> {
        let span = info_span!("audit.get_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.get(&input.key).await;
        event!(Level::INFO, op = "get", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_get_key_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = GetKeyInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_get_key_use_case_new() {
        let mock_repo = Arc::new(MockBasicRepository::new());
        let _use_case = GetKeyUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_get_key_use_case_execute_success() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_value = Value::String(b"test_value".to_vec());

        mock_repo
            .expect_get()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(Value::String(b"test_value".to_vec()))));

        let _use_case = GetKeyUseCase::new(Arc::new(mock_repo));
        let input = GetKeyInput::new(key);

        let result = _use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(expected_value));
    }

    #[tokio::test]
    async fn test_get_key_use_case_execute_not_found() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("nonexistent".to_string()).unwrap();

        mock_repo
            .expect_get()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let _use_case = GetKeyUseCase::new(Arc::new(mock_repo));
        let input = GetKeyInput::new(key);

        let result = _use_case.execute(input).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_get_key_use_case_execute_error() {
        let mut mock_repo = MockBasicRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_get()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let _use_case = GetKeyUseCase::new(Arc::new(mock_repo));
        let input = GetKeyInput::new(key);

        let result = _use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
