use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HDelInput {
    pub key: Key,
    pub fields: Vec<String>,
}

impl HDelInput {
    pub fn new(key: Key, fields: Vec<String>) -> Self {
        Self { key, fields }
    }
}

pub struct HDelUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HDelUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HDelInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.hdel_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.hdel(&input.key, input.fields).await;
        event!(Level::INFO, op = "hdel", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hdel_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let fields = vec!["field1".to_string(), "field2".to_string()];
        let input = HDelInput::new(key.clone(), fields.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.fields, fields);
    }

    #[tokio::test]
    async fn test_hdel_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HDelUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hdel_use_case_execute_success() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let fields = vec!["field1".to_string(), "field2".to_string()];

        mock_repo
            .expect_hdel()
            .with(eq(key.clone()), eq(fields.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = HDelUseCase::new(Arc::new(mock_repo));
        let input = HDelInput::new(key, fields);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_hdel_use_case_execute_partial_success() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let fields = vec!["existing".to_string(), "nonexistent".to_string()];

        mock_repo
            .expect_hdel()
            .with(eq(key.clone()), eq(fields.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = HDelUseCase::new(Arc::new(mock_repo));
        let input = HDelInput::new(key, fields);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_hdel_use_case_execute_no_fields_deleted() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let fields = vec!["nonexistent".to_string()];

        mock_repo
            .expect_hdel()
            .with(eq(key.clone()), eq(fields.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = HDelUseCase::new(Arc::new(mock_repo));
        let input = HDelInput::new(key, fields);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_hdel_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let fields = vec!["error_field".to_string()];

        mock_repo
            .expect_hdel()
            .with(eq(key.clone()), eq(fields.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HDelUseCase::new(Arc::new(mock_repo));
        let input = HDelInput::new(key, fields);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
