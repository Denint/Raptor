use crate::domain::{errors::DomainError, repositories::ListRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct RPushInput {
    pub key: Key,
    pub values: Vec<Vec<u8>>,
}

impl RPushInput {
    pub fn new(key: Key, values: Vec<Vec<u8>>) -> Self {
        Self { key, values }
    }
}

pub struct RPushUseCase<R: ?Sized + ListRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ListRepository> RPushUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: RPushInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.rpush_use_case", key = %input.key.as_str(), values_len = input.values.len());
        let _enter = span.enter();
        let result = self.repository.rpush(&input.key, input.values).await;
        event!(Level::INFO, op = "rpush", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_rpush_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![b"value1".to_vec(), b"value2".to_vec()];
        let input = RPushInput::new(key.clone(), values.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.values, values);
    }

    #[tokio::test]
    async fn test_rpush_use_case_new() {
        let mock_repo = Arc::new(MockListRepository::new());
        let _use_case = RPushUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_rpush_use_case_execute_success() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![b"value1".to_vec(), b"value2".to_vec()];

        mock_repo
            .expect_rpush()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = RPushUseCase::new(Arc::new(mock_repo));
        let input = RPushInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_rpush_use_case_execute_single_value() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let values = vec![b"single_value".to_vec()];

        mock_repo
            .expect_rpush()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = RPushUseCase::new(Arc::new(mock_repo));
        let input = RPushInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_rpush_use_case_execute_multiple_values() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("multi_key".to_string()).unwrap();
        let values = vec![b"first".to_vec(), b"second".to_vec(), b"third".to_vec()];

        mock_repo
            .expect_rpush()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(3));

        let use_case = RPushUseCase::new(Arc::new(mock_repo));
        let input = RPushInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_rpush_use_case_execute_empty_values() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let values: Vec<Vec<u8>> = vec![];

        mock_repo
            .expect_rpush()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = RPushUseCase::new(Arc::new(mock_repo));
        let input = RPushInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_rpush_use_case_execute_error() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let values = vec![b"error_value".to_vec()];

        mock_repo
            .expect_rpush()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = RPushUseCase::new(Arc::new(mock_repo));
        let input = RPushInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
