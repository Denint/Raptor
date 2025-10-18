use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HLenInput {
    pub key: Key,
}

impl HLenInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct HLenUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HLenUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HLenInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.hlen_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.hlen(&input.key).await;
        event!(Level::INFO, op = "hlen", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hlen_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = HLenInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_hlen_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HLenUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hlen_use_case_execute_with_fields() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_hlen()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(3));

        let use_case = HLenUseCase::new(Arc::new(mock_repo));
        let input = HLenInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_hlen_use_case_execute_empty_hash() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();

        mock_repo
            .expect_hlen()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(0));

        let use_case = HLenUseCase::new(Arc::new(mock_repo));
        let input = HLenInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_hlen_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_hlen()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HLenUseCase::new(Arc::new(mock_repo));
        let input = HLenInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
