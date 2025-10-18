use crate::domain::{
    errors::DomainError,
    repositories::ArrayRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ArraySetInput {
    pub key: Key,
    pub values: Vec<Value>,
}

impl ArraySetInput {
    pub fn new(key: Key, values: Vec<Value>) -> Self {
        Self { key, values }
    }
}

pub struct ArraySetUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArraySetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArraySetInput) -> Result<(), DomainError> {
        let span = info_span!("audit.array_set_use_case", key = %input.key.as_str(), array_len = input.values.len());
        let _enter = span.enter();
        let result = self.repository.array_set(&input.key, input.values).await;
        event!(Level::INFO, op = "array_set", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_set_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![Value::String(b"value1".to_vec()), Value::Integer(42)];
        let input = ArraySetInput::new(key.clone(), values.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.values, values);
    }

    #[tokio::test]
    async fn test_array_set_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArraySetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_success() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let values = vec![Value::String(b"value1".to_vec()), Value::Integer(42)];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_single_value() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let values = vec![Value::String(b"single_value".to_vec())];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_multiple_values() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("multi_key".to_string()).unwrap();
        let values = vec![
            Value::String(b"first".to_vec()),
            Value::Integer(1),
            Value::String(b"third".to_vec()),
        ];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_empty_values() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let values: Vec<Value> = vec![];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_overwrite_existing() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("existing_key".to_string()).unwrap();
        let values = vec![Value::String(b"new_value".to_vec())];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Ok(()));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_array_set_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let values = vec![Value::String(b"error_value".to_vec())];

        mock_repo
            .expect_array_set()
            .with(eq(key.clone()), eq(values.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArraySetUseCase::new(Arc::new(mock_repo));
        let input = ArraySetInput::new(key, values);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
