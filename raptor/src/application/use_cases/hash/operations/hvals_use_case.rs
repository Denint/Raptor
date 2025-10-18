use crate::domain::{errors::DomainError, repositories::HashRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct HValsInput {
    pub key: Key,
}

impl HValsInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct HValsUseCase<R: ?Sized + HashRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + HashRepository> HValsUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: HValsInput) -> Result<Vec<Vec<u8>>, DomainError> {
        let span = info_span!("audit.hvals_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.hvals(&input.key).await;
        event!(Level::INFO, op = "hvals", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_hvals_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = HValsInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_hvals_use_case_new() {
        let mock_repo = Arc::new(MockHashRepository::new());
        let _use_case = HValsUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_hvals_use_case_execute_with_values() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values = vec![b"value1".to_vec(), b"value2".to_vec(), b"value3".to_vec()];

        mock_repo
            .expect_hvals()
            .with(eq(key.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = HValsUseCase::new(Arc::new(mock_repo));
        let input = HValsInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], b"value1");
        assert_eq!(result[1], b"value2");
        assert_eq!(result[2], b"value3");
    }

    #[tokio::test]
    async fn test_hvals_use_case_execute_empty_hash() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let expected_values = Vec::<Vec<u8>>::new();

        mock_repo
            .expect_hvals()
            .with(eq(key.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = HValsUseCase::new(Arc::new(mock_repo));
        let input = HValsInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Vec::<Vec<u8>>::new());
    }

    #[tokio::test]
    async fn test_hvals_use_case_execute_error() {
        let mut mock_repo = MockHashRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_hvals()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = HValsUseCase::new(Arc::new(mock_repo));
        let input = HValsInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
