use crate::domain::{
    errors::DomainError,
    repositories::ArrayRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;

pub struct ArrayAppendInput {
    pub key: Key,
    pub values: Vec<Value>,
}

impl ArrayAppendInput {
    pub fn new(key: Key, values: Vec<Value>) -> Self {
        Self { key, values }
    }
}

pub struct ArrayAppendUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArrayAppendUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArrayAppendInput) -> Result<usize, DomainError> {
        self.repository.array_append(&input.key, input.values).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_append_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![Value::String(b"value1".to_vec()), Value::Integer(42)];
        let input = ArrayAppendInput::new(key.clone(), values.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.values, values);
    }

    #[tokio::test]
    async fn test_array_append_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArrayAppendUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_append_use_case_execute_success() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![Value::String(b"value1".to_vec()), Value::Integer(42)];

        mock_repo
            .expect_array_append()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = ArrayAppendUseCase::new(Arc::new(mock_repo));
        let input = ArrayAppendInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_array_append_use_case_execute_single_value() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let values = vec![Value::String(b"single_value".to_vec())];

        mock_repo
            .expect_array_append()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = ArrayAppendUseCase::new(Arc::new(mock_repo));
        let input = ArrayAppendInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_array_append_use_case_execute_multiple_values() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("multi_key".to_string()).unwrap();
        let values = vec![
            Value::String(b"first".to_vec()),
            Value::Integer(1),
            Value::String(b"third".to_vec()),
        ];

        mock_repo
            .expect_array_append()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(3));

        let use_case = ArrayAppendUseCase::new(Arc::new(mock_repo));
        let input = ArrayAppendInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_array_append_use_case_execute_empty_values() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let values: Vec<Value> = vec![];

        mock_repo
            .expect_array_append()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = ArrayAppendUseCase::new(Arc::new(mock_repo));
        let input = ArrayAppendInput::new(key, values);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_array_append_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let values = vec![Value::String(b"error_value".to_vec())];

        mock_repo
            .expect_array_append()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArrayAppendUseCase::new(Arc::new(mock_repo));
        let input = ArrayAppendInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
