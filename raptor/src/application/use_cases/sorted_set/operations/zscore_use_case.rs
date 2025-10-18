use crate::domain::{
    errors::DomainError, repositories::SortedSetRepository, value_objects::key::Key,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ZScoreInput {
    pub key: Key,
    pub member: Vec<u8>,
}

impl ZScoreInput {
    pub fn new(key: Key, member: Vec<u8>) -> Self {
        Self { key, member }
    }
}

pub struct ZScoreUseCase<R: ?Sized + SortedSetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SortedSetRepository> ZScoreUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ZScoreInput) -> Result<Option<f64>, DomainError> {
        let span = info_span!("audit.zscore_use_case", key = %input.key.as_str());
        let _enter = span.enter();
        let result = self.repository.zscore(&input.key, &input.member).await;
        event!(Level::INFO, op = "zscore", key = %input.key.as_str(), member = %String::from_utf8_lossy(&input.member), success = result.is_ok());
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
    async fn test_zscore_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"test_member".to_vec();
        let input = ZScoreInput::new(key.clone(), member.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.member, member);
    }

    #[tokio::test]
    async fn test_zscore_use_case_new() {
        let mock_repo = Arc::new(MockSortedSetRepository::new());
        let _use_case = ZScoreUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_member_exists() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"existing_member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(Some(2.5)));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(2.5));
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_member_not_exists() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = b"nonexistent_member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(None));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_zero_score() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("zero_score_key".to_string()).unwrap();
        let member = b"zero_score_member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(Some(0.0)));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(0.0));
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_negative_score() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("negative_score_key".to_string()).unwrap();
        let member = b"negative_score_member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(Some(-5.5)));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, Some(-5.5));
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_empty_member() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let member = vec![];

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(None));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_key_not_exists() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("nonexistent_key".to_string()).unwrap();
        let member = b"member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Ok(None));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_zscore_use_case_execute_error() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let member = b"error_member".to_vec();

        mock_repo
            .expect_zscore()
            .with(eq(key.clone()), eq(member.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ZScoreUseCase::new(Arc::new(mock_repo));
        let input = ZScoreInput::new(key, member);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
