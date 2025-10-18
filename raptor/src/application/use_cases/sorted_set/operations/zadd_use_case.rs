use crate::domain::{
    errors::DomainError, repositories::SortedSetRepository, value_objects::key::Key,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ZAddInput {
    pub key: Key,
    pub score: f64,
    pub member: Vec<u8>,
}

impl ZAddInput {
    pub fn new(key: Key, score: f64, member: Vec<u8>) -> Self {
        Self { key, score, member }
    }
}

pub struct ZAddUseCase<R: ?Sized + SortedSetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SortedSetRepository> ZAddUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ZAddInput) -> Result<bool, DomainError> {
        let span =
            info_span!("audit.zadd_use_case", key = %input.key.as_str(), score = input.score);
        let _enter = span.enter();
        let result = self
            .repository
            .zadd(&input.key, input.score, input.member)
            .await;
        event!(Level::INFO, op = "zadd", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockSortedSetRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_zadd_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let score = 1.5;
        let member = b"test_member".to_vec();
        let input = ZAddInput::new(key.clone(), score, member.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.score, score);
        assert_eq!(input.member, member);
    }

    #[tokio::test]
    async fn test_zadd_use_case_new() {
        let mock_repo = Arc::new(MockSortedSetRepository::new());
        let _use_case = ZAddUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_success() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let score = 2.5;
        let member = b"new_member".to_vec();

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Ok(true));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_update_existing() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let score = 3.0;
        let member = b"existing_member".to_vec();

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Ok(false));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_zero_score() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("zero_score_key".to_string()).unwrap();
        let score = 0.0;
        let member = b"zero_score_member".to_vec();

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Ok(true));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_negative_score() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("negative_score_key".to_string()).unwrap();
        let score = -5.5;
        let member = b"negative_score_member".to_vec();

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Ok(true));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_empty_member() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("empty_member_key".to_string()).unwrap();
        let score = 1.0;
        let member = vec![];

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Ok(true));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await.unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_zadd_use_case_execute_error() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let score = 1.5;
        let member = b"error_member".to_vec();

        mock_repo
            .expect_zadd()
            .with(eq(key.clone()), eq(score), eq(member.clone()))
            .times(1)
            .returning(|_, _, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ZAddUseCase::new(Arc::new(mock_repo));
        let input = ZAddInput::new(key, score, member);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
