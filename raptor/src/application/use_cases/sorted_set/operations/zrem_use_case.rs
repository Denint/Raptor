use crate::domain::{
    errors::DomainError, repositories::SortedSetRepository, value_objects::key::Key,
};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct ZRemInput {
    pub key: Key,
    pub members: Vec<Vec<u8>>,
}

impl ZRemInput {
    pub fn new(key: Key, members: Vec<Vec<u8>>) -> Self {
        Self { key, members }
    }
}

pub struct ZRemUseCase<R: ?Sized + SortedSetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SortedSetRepository> ZRemUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ZRemInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.zrem_use_case", key = %input.key.as_str(), members_len = input.members.len());
        let _enter = span.enter();
        let result = self.repository.zrem(&input.key, input.members).await;
        event!(Level::INFO, op = "zrem", key = %input.key.as_str(), success = result.is_ok());
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
    async fn test_zrem_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let members = vec![b"member1".to_vec(), b"member2".to_vec()];
        let input = ZRemInput::new(key.clone(), members.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.members, members);
    }

    #[tokio::test]
    async fn test_zrem_use_case_new() {
        let mock_repo = Arc::new(MockSortedSetRepository::new());
        let _use_case = ZRemUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_success() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let members = vec![b"member1".to_vec(), b"member2".to_vec()];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_single_member() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let members = vec![b"single_member".to_vec()];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_partial_removal() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("partial_key".to_string()).unwrap();
        let members = vec![b"existing".to_vec(), b"nonexistent".to_vec()];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_no_removal() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("no_removal_key".to_string()).unwrap();
        let members = vec![b"nonexistent1".to_vec(), b"nonexistent2".to_vec()];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_empty_members() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let members: Vec<Vec<u8>> = vec![];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_zrem_use_case_execute_error() {
        let mut mock_repo = MockSortedSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let members = vec![b"error_member".to_vec()];

        mock_repo
            .expect_zrem()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ZRemUseCase::new(Arc::new(mock_repo));
        let input = ZRemInput::new(key, members);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
