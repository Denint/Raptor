use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HGetInput {
    pub key: Key,
    pub field: String,
}

impl HGetInput {
    pub fn new(key: Key, field: String) -> Self {
        Self { key, field }
    }
}

pub struct HGetUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HGetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HGetInput) -> Result<Option<Vec<u8>>, DomainError> {
        let span =
            info_span!("audit.hget_use_case", key = %input.key.as_str(), field = %input.field);
        let _enter = span.enter();
        let result = self.repository.hget(&input.key, &input.field).await;
        event!(Level::INFO, op = "hget", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hget_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let input = HGetInput::new(key.clone(), field.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.field, field);
    }

    #[tokio::test]
    async fn test_hget_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HGetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hget_use_case_execute_found() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let expected_value = b"test_value".to_vec();

        mock_repo
            .expect_hget()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(move |_, _| Ok(Some(expected_value.clone())));

        let use_case = HGetUseCase::new(Arc::new(mock_repo));
        let input = HGetInput::new(key, field);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(b"test_value".to_vec()));
    }

    #[tokio::test]
    async fn test_hget_use_case_execute_not_found() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "nonexistent_field".to_string();

        mock_repo
            .expect_hget()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(|_, _| Ok(None));

        let use_case = HGetUseCase::new(Arc::new(mock_repo));
        let input = HGetInput::new(key, field);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_hget_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let field = "error_field".to_string();

        mock_repo
            .expect_hget()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HGetUseCase::new(Arc::new(mock_repo));
        let input = HGetInput::new(key, field);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
