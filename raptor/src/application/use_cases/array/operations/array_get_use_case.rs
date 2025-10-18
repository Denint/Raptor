use crate::domain::{
    errors::DomainError,
    repositories::ArrayRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;

pub struct ArrayGetInput {
    pub key: Key,
    pub indices: Vec<usize>,
}

impl ArrayGetInput {
    pub fn new(key: Key, indices: Vec<usize>) -> Self {
        Self { key, indices }
    }
}

pub struct ArrayGetUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArrayGetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArrayGetInput) -> Result<Vec<Option<Value>>, DomainError> {
        self.repository.array_get(&input.key, input.indices).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_get_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let indices = vec![0, 2, 4];
        let input = ArrayGetInput::new(key.clone(), indices.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.indices, indices);
    }

    #[tokio::test]
    async fn test_array_get_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArrayGetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_success() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let indices = vec![0, 2];
        let expected_values = vec![
            Some(Value::String(b"value0".to_vec())),
            Some(Value::Integer(42)),
        ];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| {
                Ok(vec![
                    Some(Value::String(b"value0".to_vec())),
                    Some(Value::Integer(42)),
                ])
            });

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_mixed_results() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("mixed_key".to_string()).unwrap();
        let indices = vec![0, 1, 5];
        let expected_values = vec![
            Some(Value::String(b"value0".to_vec())),
            None,
            Some(Value::Integer(100)),
        ];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| {
                Ok(vec![
                    Some(Value::String(b"value0".to_vec())),
                    None,
                    Some(Value::Integer(100)),
                ])
            });

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_single_index() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let indices = vec![1];
        let expected_values = vec![Some(Value::String(b"single_value".to_vec()))];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| Ok(vec![Some(Value::String(b"single_value".to_vec()))]));

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_empty_indices() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let indices: Vec<usize> = vec![];
        let expected_values: Vec<Option<Value>> = vec![];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| Ok(vec![]));

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_all_none() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("none_key".to_string()).unwrap();
        let indices = vec![0, 1, 2];
        let expected_values = vec![None, None, None];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| Ok(vec![None, None, None]));

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, expected_values);
    }

    #[tokio::test]
    async fn test_array_get_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let indices = vec![0];

        mock_repo
            .expect_array_get()
            .with(eq(key.clone()), eq(indices.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArrayGetUseCase::new(Arc::new(mock_repo));
        let input = ArrayGetInput::new(key, indices);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
