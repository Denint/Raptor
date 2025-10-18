use crate::domain::{errors::DomainError, repositories::ListRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct LPopInput {
    pub key: Key,
}

impl LPopInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct LPopUseCase<R: ?Sized + ListRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ListRepository> LPopUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: LPopInput) -> Result<Option<Vec<u8>>, DomainError> {
        let span = info_span!("audit.lpop_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.lpop(&input.key).await;
        event!(Level::INFO, op = "lpop", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockListRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_lpop_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = LPopInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_lpop_use_case_new() {
        let mock_repo = Arc::new(MockListRepository::new());
        let _use_case = LPopUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_lpop_use_case_execute_success() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_lpop()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(b"popped_value".to_vec())));

        let use_case = LPopUseCase::new(Arc::new(mock_repo));
        let input = LPopInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(b"popped_value".to_vec()));
    }

    #[tokio::test]
    async fn test_lpop_use_case_execute_empty_list() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();

        mock_repo
            .expect_lpop()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = LPopUseCase::new(Arc::new(mock_repo));
        let input = LPopInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_lpop_use_case_execute_nonexistent_key() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();

        mock_repo
            .expect_lpop()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = LPopUseCase::new(Arc::new(mock_repo));
        let input = LPopInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_lpop_use_case_execute_error() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_lpop()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = LPopUseCase::new(Arc::new(mock_repo));
        let input = LPopInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }

    #[tokio::test]
    async fn test_lpop_use_case_execute_different_value_types() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("value_key".to_string()).unwrap();

        mock_repo
            .expect_lpop()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(vec![])));

        let use_case = LPopUseCase::new(Arc::new(mock_repo));
        let input = LPopInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(vec![]));
    }
}
