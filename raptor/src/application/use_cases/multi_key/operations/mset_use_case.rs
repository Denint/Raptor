use crate::domain::{
    errors::DomainError,
    repositories::MultiRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct MSetInput {
    pub pairs: Vec<(Key, Value)>,
}

impl MSetInput {
    pub fn new(pairs: Vec<(Key, Value)>) -> Self {
        Self { pairs }
    }
}

pub struct MSetUseCase<R: ?Sized + MultiRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + MultiRepository> MSetUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: MSetInput) -> Result<(), DomainError> {
        let span = info_span!("audit.mset_use_case", pairs_len = input.pairs.len());
        let _enter = span.enter();
        let result = self.repository.mset(input.pairs).await;
        event!(Level::INFO, op = "mset", success = result.is_ok());
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
    async fn test_mset_input_new() {
        let pairs = vec![
            (
                Key::new("key1".to_string()).unwrap(),
                Value::String(b"value1".to_vec()),
            ),
            (Key::new("key2".to_string()).unwrap(), Value::Integer(42)),
        ];
        let input = MSetInput::new(pairs.clone());
        assert_eq!(input.pairs, pairs);
    }

    #[tokio::test]
    async fn test_mset_use_case_new() {
        let mock_repo = Arc::new(MockMultiRepository::new());
        let _use_case = MSetUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_mset_use_case_execute_success() {
        let mut mock_repo = MockMultiRepository::new();
        let pairs = vec![
            (
                Key::new("key1".to_string()).unwrap(),
                Value::String(b"value1".to_vec()),
            ),
            (Key::new("key2".to_string()).unwrap(), Value::Integer(42)),
            (
                Key::new("key3".to_string()).unwrap(),
                Value::String(b"value3".to_vec()),
            ),
        ];

        mock_repo
            .expect_mset()
            .with(eq(pairs.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let use_case = MSetUseCase::new(Arc::new(mock_repo));
        let input = MSetInput::new(pairs);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mset_use_case_execute_single_pair() {
        let mut mock_repo = MockMultiRepository::new();
        let pairs = vec![(
            Key::new("single_key".to_string()).unwrap(),
            Value::String(b"single_value".to_vec()),
        )];

        mock_repo
            .expect_mset()
            .with(eq(pairs.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let use_case = MSetUseCase::new(Arc::new(mock_repo));
        let input = MSetInput::new(pairs);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mset_use_case_execute_empty_pairs() {
        let mut mock_repo = MockMultiRepository::new();
        let pairs: Vec<(Key, Value)> = vec![];

        mock_repo
            .expect_mset()
            .with(eq(pairs.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let use_case = MSetUseCase::new(Arc::new(mock_repo));
        let input = MSetInput::new(pairs);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mset_use_case_execute_overwrite_existing() {
        let mut mock_repo = MockMultiRepository::new();
        let pairs = vec![(
            Key::new("existing_key".to_string()).unwrap(),
            Value::String(b"new_value".to_vec()),
        )];

        mock_repo
            .expect_mset()
            .with(eq(pairs.clone()))
            .times(1)
            .returning(|_| Ok(()));

        let use_case = MSetUseCase::new(Arc::new(mock_repo));
        let input = MSetInput::new(pairs);

        let result = use_case.execute(input).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_mset_use_case_execute_error() {
        let mut mock_repo = MockMultiRepository::new();
        let pairs = vec![(
            Key::new("error_key".to_string()).unwrap(),
            Value::String(b"error_value".to_vec()),
        )];

        mock_repo
            .expect_mset()
            .with(eq(pairs.clone()))
            .times(1)
            .returning(|_| Err(DomainError::StorageError("test error".to_string())));

        let use_case = MSetUseCase::new(Arc::new(mock_repo));
        let input = MSetInput::new(pairs);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
