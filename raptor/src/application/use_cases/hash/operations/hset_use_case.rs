use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HSetInput {
    pub key: Key,
    pub field: String,
    pub value: Vec<u8>,
}

impl HSetInput {
    pub fn new(key: Key, field: String, value: Vec<u8>) -> Self {
        Self { key, field, value }
    }
}

pub struct HSetUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HSetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HSetInput) -> Result<bool, DomainError> {
        let span =
            info_span!("audit.hset_use_case", key = %input.key.as_str(), field = %input.field);
        let _enter = span.enter();
        let result = self
            .repository
            .hset(&input.key, input.field, input.value)
            .await;
        event!(Level::INFO, op = "hset", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockHashRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_hset_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let value = b"test_value".to_vec();
        let input = HSetInput::new(key.clone(), field.clone(), value.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.field, field);
        assert_eq!(input.value, value);
    }

    #[tokio::test]
    async fn test_hset_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HSetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hset_use_case_execute_success() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let value = b"test_value".to_vec();

        mock_repo
            .expect_hset()
            .with(eq(key.clone()), eq(field.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _, _| Ok(true));

        let use_case = HSetUseCase::new(Arc::new(mock_repo));
        let input = HSetInput::new(key, field, value);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_hset_use_case_execute_update() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let value = b"updated_value".to_vec();

        mock_repo
            .expect_hset()
            .with(eq(key.clone()), eq(field.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _, _| Ok(false));

        let use_case = HSetUseCase::new(Arc::new(mock_repo));
        let input = HSetInput::new(key, field, value);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_hset_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let field = "error_field".to_string();
        let value = b"error_value".to_vec();

        mock_repo
            .expect_hset()
            .with(eq(key.clone()), eq(field.clone()), eq(value.clone()))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HSetUseCase::new(Arc::new(mock_repo));
        let input = HSetInput::new(key, field, value);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
