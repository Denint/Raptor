use crate::domain::{
    errors::DomainError, repositories::SortedSetRepository, value_objects::key::Key,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ZRangeInput {
    pub key: Key,
    pub start: isize,
    pub stop: isize,
}

impl ZRangeInput {
    pub fn new(key: Key, start: isize, stop: isize) -> Self {
        Self { key, start, stop }
    }
}

pub struct ZRangeUseCase<R: ?Sized + SortedSetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SortedSetRepository> ZRangeUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ZRangeInput) -> Result<Vec<Vec<u8>>, DomainError> {
        let span = info_span!("audit.zrange_use_case", key = %input.key.as_str(), start = input.start, stop = input.stop);
        let _enter = span.enter();
        let result = self
            .repository
            .zrange(&input.key, input.start, input.stop)
            .await;
        event!(Level::INFO, op = "zrange", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockSortedSetRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_zrange_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ZRangeInput::new(key.clone(), 0, 10);
        assert_eq!(input.key, key);
        assert_eq!(input.start, 0);
        assert_eq!(input.stop, 10);
    }

    #[tokio::test]
    async fn test_zrange_use_case_new() {
        let mock_repo = Arc::new(MockSortedSetRepository::new());
        let _use_case = ZRangeUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_zrange_use_case_execute_success() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_zrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Ok(vec![b"value1".to_vec(), b"value2".to_vec()]));

        let use_case = ZRangeUseCase::new(Arc::new(mock_repo));
        let input = ZRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, vec![b"value1".to_vec(), b"value2".to_vec()]);
    }

    #[tokio::test]
    async fn test_zrange_use_case_execute_empty_result() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_zrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Ok(vec![]));

        let use_case = ZRangeUseCase::new(Arc::new(mock_repo));
        let input = ZRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[tokio::test]
    async fn test_zrange_use_case_execute_negative_indices() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_zrange()
            .with(eq(key.clone()), eq(-5isize), eq(-1isize))
            .times(1)
            .returning(|_, _, _| Ok(vec![b"last5".to_vec(), b"last4".to_vec()]));

        let use_case = ZRangeUseCase::new(Arc::new(mock_repo));
        let input = ZRangeInput::new(key, -5, -1);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, vec![b"last5".to_vec(), b"last4".to_vec()]);
    }

    #[tokio::test]
    async fn test_zrange_use_case_execute_error() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_zrange()
            .with(eq(key.clone()), eq(0isize), eq(10isize))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ZRangeUseCase::new(Arc::new(mock_repo));
        let input = ZRangeInput::new(key, 0, 10);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
