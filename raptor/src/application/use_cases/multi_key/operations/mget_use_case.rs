use crate::domain::{
    errors::DomainError,
    repositories::MultiRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct MGetInput {
    pub keys: Vec<Key>,
}

impl MGetInput {
    pub fn new(keys: Vec<Key>) -> Self {
        Self { keys }
    }
}

pub struct MGetUseCase<R: ?Sized + MultiRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + MultiRepository> MGetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: MGetInput) -> Result<Vec<Option<Value>>, DomainError> {
        let span = info_span!("audit.mget_use_case", keys_len = input.keys.len());
        let _enter = span.enter();
        let result = self.repository.mget(input.keys).await;
        event!(Level::INFO, op = "mget", success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockMultiRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_mget_input_new() {
        let keys = vec![
            Key::new("key1".to_string()).unwrap(),
            Key::new("key2".to_string()).unwrap(),
        ];
        let input = MGetInput::new(keys.clone());
        assert_eq!(input.keys, keys);
    }

    #[tokio::test]
    async fn test_mget_use_case_new() {
        let mock_repo = Arc::new(MockMultiRepository::new());
        let _use_case = MGetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_mget_use_case_execute_mixed_results() {
        let mut mock_repo = MockMultiRepository::new();
        let keys = vec![
            Key::new("existing_key".to_string()).unwrap(),
            Key::new("missing_key".to_string()).unwrap(),
            Key::new("another_existing_key".to_string()).unwrap(),
        ];
        let expected_values = vec![
            Some(Value::String(b"value1".to_vec())),
            None,
            Some(Value::Integer(42)),
        ];

        mock_repo
            .expect_mget()
            .with(eq(keys.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = MGetUseCase::new(Arc::new(mock_repo));
        let input = MGetInput::new(keys);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Some(Value::String(b"value1".to_vec())));
        assert_eq!(result[1], None);
        assert_eq!(result[2], Some(Value::Integer(42)));
    }

    #[tokio::test]
    async fn test_mget_use_case_execute_all_found() {
        let mut mock_repo = MockMultiRepository::new();
        let keys = vec![
            Key::new("key1".to_string()).unwrap(),
            Key::new("key2".to_string()).unwrap(),
        ];
        let expected_values = vec![
            Some(Value::String(b"value1".to_vec())),
            Some(Value::Integer(42)),
        ];

        mock_repo
            .expect_mget()
            .with(eq(keys.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = MGetUseCase::new(Arc::new(mock_repo));
        let input = MGetInput::new(keys);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Some(Value::String(b"value1".to_vec())));
        assert_eq!(result[1], Some(Value::Integer(42)));
    }

    #[tokio::test]
    async fn test_mget_use_case_execute_all_missing() {
        let mut mock_repo = MockMultiRepository::new();
        let keys = vec![
            Key::new("missing1".to_string()).unwrap(),
            Key::new("missing2".to_string()).unwrap(),
        ];
        let expected_values = vec![None, None];

        mock_repo
            .expect_mget()
            .with(eq(keys.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = MGetUseCase::new(Arc::new(mock_repo));
        let input = MGetInput::new(keys);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, vec![None, None]);
    }

    #[tokio::test]
    async fn test_mget_use_case_execute_empty_keys() {
        let mut mock_repo = MockMultiRepository::new();
        let keys: Vec<Key> = vec![];
        let expected_values: Vec<Option<Value>> = vec![];

        mock_repo
            .expect_mget()
            .with(eq(keys.clone()))
            .times(1)
            .returning(move |_| Ok(expected_values.clone()));

        let use_case = MGetUseCase::new(Arc::new(mock_repo));
        let input = MGetInput::new(keys);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Vec::<Option<Value>>::new());
    }

    #[tokio::test]
    async fn test_mget_use_case_execute_error() {
        let mut mock_repo = MockMultiRepository::new();
        let keys = vec![Key::new("error_key".to_string()).unwrap()];

        mock_repo
            .expect_mget()
            .with(eq(keys.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = MGetUseCase::new(Arc::new(mock_repo));
        let input = MGetInput::new(keys);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
