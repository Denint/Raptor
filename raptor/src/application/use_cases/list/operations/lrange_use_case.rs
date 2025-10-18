use crate::domain::{errors::DomainError, repositories::ListRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct LRangeInput {
    pub key: Key,
    pub start: isize,
    pub stop: isize,
}

impl LRangeInput {
    pub fn new(key: Key, start: isize, stop: isize) -> Self {
        Self { key, start, stop }
    }
}

pub struct LRangeUseCase<R: ?Sized + ListRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ListRepository> LRangeUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: LRangeInput) -> Result<Vec<Vec<u8>>, DomainError> {
        let span = info_span!("audit.lrange_use_case", key = %input.key.as_str(), start = input.start, stop = input.stop);
        let _enter = span.enter();
        let result = self
            .repository
            .lrange(&input.key, input.start, input.stop)
            .await;
        event!(Level::INFO, op = "lrange", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_lrange_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = LRangeInput::new(key.clone(), 0, 10);
        assert_eq!(input.key, key);
        assert_eq!(input.start, 0);
        assert_eq!(input.stop, 10);
    }

    #[tokio::test]
    async fn test_lrange_use_case_new() {
        let mock_repo = Arc::new(MockListRepository::new());
        let _use_case = LRangeUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_lrange_use_case_execute_success() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_lrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Ok(vec![b"value1".to_vec(), b"value2".to_vec()]));

        let use_case = LRangeUseCase::new(Arc::new(mock_repo));
        let input = LRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, vec![b"value1".to_vec(), b"value2".to_vec()]);
    }

    #[tokio::test]
    async fn test_lrange_use_case_execute_empty_result() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_lrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Ok(vec![]));

        let use_case = LRangeUseCase::new(Arc::new(mock_repo));
        let input = LRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[tokio::test]
    async fn test_lrange_use_case_execute_error() {
        let mut mock_repo = MockListRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_lrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = LRangeUseCase::new(Arc::new(mock_repo));
        let input = LRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
