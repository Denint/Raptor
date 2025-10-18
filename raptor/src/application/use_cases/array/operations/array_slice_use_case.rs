use crate::domain::{
    errors::DomainError,
    repositories::ArrayRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;

pub struct ArraySliceInput {
    pub key: Key,
    pub start: usize,
    pub end: Option<usize>,
}

impl ArraySliceInput {
    pub fn new(key: Key, start: usize, end: Option<usize>) -> Self {
        Self { key, start, end }
    }
}

pub struct ArraySliceUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArraySliceUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArraySliceInput) -> Result<Vec<Value>, DomainError> {
        self.repository
            .array_slice(&input.key, input.start, input.end)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_slice_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ArraySliceInput::new(key.clone(), 0, Some(10));
        assert_eq!(input.key, key);
        assert_eq!(input.start, 0);
        assert_eq!(input.end, Some(10));
    }

    #[tokio::test]
    async fn test_array_slice_input_new_without_end() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ArraySliceInput::new(key.clone(), 5, None);
        assert_eq!(input.key, key);
        assert_eq!(input.start, 5);
        assert_eq!(input.end, None);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArraySliceUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_with_end() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values = vec![Value::String(b"value1".to_vec()), Value::Integer(42)];

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(0usize), eq(Some(2usize)))
            .times(1)
            .returning(|_, _, _| Ok(vec![Value::String(b"value1".to_vec()), Value::Integer(42)]));

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 0, Some(2));

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_without_end() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values = vec![Value::String(b"value".to_vec())];

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(1usize), eq(None))
            .times(1)
            .returning(|_, _, _| Ok(vec![Value::String(b"value".to_vec())]));

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 1, None);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_empty_slice() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values: Vec<Value> = vec![];

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(10usize), eq(Some(5usize)))
            .times(1)
            .returning(|_, _, _| Ok(vec![]));

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 10, Some(5));

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_full_slice() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values = vec![
            Value::String(b"first".to_vec()),
            Value::Integer(2),
            Value::String(b"third".to_vec()),
        ];

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(0usize), eq(None))
            .times(1)
            .returning(|_, _, _| {
                Ok(vec![
                    Value::String(b"first".to_vec()),
                    Value::Integer(2),
                    Value::String(b"third".to_vec()),
                ])
            });

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 0, None);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_single_element() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let expected_values = vec![Value::String(b"single".to_vec())];

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(2usize), eq(Some(3usize)))
            .times(1)
            .returning(|_, _, _| Ok(vec![Value::String(b"single".to_vec())]));

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 2, Some(3));

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_slice_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_array_slice()
            .with(eq(key.clone()), eq(0usize), eq(Some(5usize)))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArraySliceUseCase::new(Arc::new(mock_repo));
        let input = ArraySliceInput::new(key, 0, Some(5));

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
