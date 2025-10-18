use crate::domain::{errors::DomainError, repositories::TtlRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ExpireInput {
    pub key: Key,
    pub ttl_seconds: u64,
}

impl ExpireInput {
    pub fn new(key: Key, ttl_seconds: u64) -> Self {
        Self { key, ttl_seconds }
    }
}

pub struct ExpireUseCase<R: ?Sized + TtlRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + TtlRepository> ExpireUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ExpireInput) -> Result<bool, DomainError> {
        let span = info_span!("audit.expire_use_case", key = %input.key.as_str(), ttl_seconds = input.ttl_seconds);
        let _enter = span.enter();
        let result = self.repository.expire(&input.key, input.ttl_seconds).await;
        event!(Level::INFO, op = "expire", key = %input.key.as_str(), success = result.is_ok(), ttl_seconds = input.ttl_seconds);
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockTtlRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_expire_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let input = ExpireInput::new(key.clone(), 3600);
        assert_eq!(input.key, key);
        assert_eq!(input.ttl_seconds, 3600);
    }

    #[tokio::test]
    async fn test_expire_use_case_new() {
        let mock_repo = Arc::new(MockTtlRepository::new());
        let _use_case = ExpireUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_expire_use_case_execute_success() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();

        mock_repo
            .expect_expire()
            .with(eq(key.clone()), eq(3600u64))
            .times(1)
            .returning(|_, _| Ok(true));

        let use_case = ExpireUseCase::new(Arc::new(mock_repo));
        let input = ExpireInput::new(key, 3600);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_expire_use_case_execute_failure() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_expire()
            .with(eq(key.clone()), eq(3600u64))
            .times(1)
            .returning(|_, _| Ok(false));

        let use_case = ExpireUseCase::new(Arc::new(mock_repo));
        let input = ExpireInput::new(key, 3600);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_expire_use_case_execute_error() {
        let mut mock_repo = MockTtlRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();

        mock_repo
            .expect_expire()
            .with(eq(key.clone()), eq(3600u64))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ExpireUseCase::new(Arc::new(mock_repo));
        let input = ExpireInput::new(key, 3600);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
