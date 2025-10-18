use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HKeysInput {
    pub key: Key,
}

impl HKeysInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct HKeysUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HKeysUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HKeysInput) -> Result<Vec<String>, DomainError> {
        let span = info_span!("audit.hkeys_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.hkeys(&input.key).await;
        event!(Level::INFO, op = "hkeys", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hkeys_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = HKeysInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_hkeys_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HKeysUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hkeys_use_case_execute_with_keys() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_keys = vec![
            "field1".to_string(),
            "field2".to_string(),
            "field3".to_string(),
        ];

        mock_repo
            .expect_hkeys()
            .with(eq(key.clone()))
            .times(1)
            .returning(move |_| Ok(expected_keys.clone()));

        let use_case = HKeysUseCase::new(Arc::new(mock_repo));
        let input = HKeysInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, vec!["field1", "field2", "field3"]);
    }

    #[tokio::test]
    async fn test_hkeys_use_case_execute_empty_hash() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let expected_keys = Vec::<String>::new();

        mock_repo
            .expect_hkeys()
            .with(eq(key.clone()))
            .times(1)
            .returning(move |_| Ok(expected_keys.clone()));

        let use_case = HKeysUseCase::new(Arc::new(mock_repo));
        let input = HKeysInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Vec::<String>::new());
    }

    #[tokio::test]
    async fn test_hkeys_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_hkeys()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HKeysUseCase::new(Arc::new(mock_repo));
        let input = HKeysInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
