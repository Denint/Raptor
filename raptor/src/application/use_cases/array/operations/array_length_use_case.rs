use crate::domain::{errors::DomainError, repositories::ArrayRepository, value_objects::key::Key};
use std::sync::Arc;

pub struct ArrayLengthInput {
    pub key: Key,
}

impl ArrayLengthInput {
    pub fn new(key: Key) -> Self {
        Self { key }
    }
}

pub struct ArrayLengthUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArrayLengthUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArrayLengthInput) -> Result<Option<usize>, DomainError> {
        self.repository.array_length(&input.key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_length_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ArrayLengthInput::new(key.clone());
        assert_eq!(input.key, key);
    }

    #[tokio::test]
    async fn test_array_length_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArrayLengthUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_length_use_case_execute_with_length() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_array_length()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(5)));

        let use_case = ArrayLengthUseCase::new(Arc::new(mock_repo));
        let input = ArrayLengthInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(5));
    }

    #[tokio::test]
    async fn test_array_length_use_case_execute_empty_array() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();

        mock_repo
            .expect_array_length()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(0)));

        let use_case = ArrayLengthUseCase::new(Arc::new(mock_repo));
        let input = ArrayLengthInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(0));
    }

    #[tokio::test]
    async fn test_array_length_use_case_execute_single_element() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();

        mock_repo
            .expect_array_length()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(Some(1)));

        let use_case = ArrayLengthUseCase::new(Arc::new(mock_repo));
        let input = ArrayLengthInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(1));
    }

    #[tokio::test]
    async fn test_array_length_use_case_execute_nonexistent_key() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();

        mock_repo
            .expect_array_length()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Ok(None));

        let use_case = ArrayLengthUseCase::new(Arc::new(mock_repo));
        let input = ArrayLengthInput::new(key);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_array_length_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_array_length()
            .with(eq(key.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArrayLengthUseCase::new(Arc::new(mock_repo));
        let input = ArrayLengthInput::new(key);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
