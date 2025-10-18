use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HExistsInput {
    pub key: Key,
    pub field: String,
}

impl HExistsInput {
    pub fn new(key: Key, field: String) -> Self {
        Self { key, field }
    }
}

pub struct HExistsUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HExistsUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HExistsInput) -> Result<bool, DomainError> {
        let span =
            info_span!("audit.hexists_use_case", key = %input.key.as_str(), field = %input.field);
        let _enter = span.enter();
        let result = self.repository.hexists(&input.key, &input.field).await;
        event!(Level::INFO, op = "hexists", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hexists_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "test_field".to_string();
        let input = HExistsInput::new(key.clone(), field.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.field, field);
    }

    #[tokio::test]
    async fn test_hexists_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HExistsUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hexists_use_case_execute_exists() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "existing_field".to_string();

        mock_repo
            .expect_hexists()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(|_, _| Ok(true));

        let use_case = HExistsUseCase::new(Arc::new(mock_repo));
        let input = HExistsInput::new(key, field);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_hexists_use_case_execute_not_exists() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let field = "nonexistent_field".to_string();

        mock_repo
            .expect_hexists()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(|_, _| Ok(false));

        let use_case = HExistsUseCase::new(Arc::new(mock_repo));
        let input = HExistsInput::new(key, field);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_hexists_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let field = "error_field".to_string();

        mock_repo
            .expect_hexists()
            .with(eq(key.clone()), eq(field.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HExistsUseCase::new(Arc::new(mock_repo));
        let input = HExistsInput::new(key, field);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
