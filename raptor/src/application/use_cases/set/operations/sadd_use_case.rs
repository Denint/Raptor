use crate::domain::{errors::DomainError, repositories::SetRepository, value_objects::key::Key};
use std::sync::Arc;
use tracing::{Level, event, info_span};

pub struct SAddInput {
    pub key: Key,
    pub members: Vec<Vec<u8>>,
}

impl SAddInput {
    pub fn new(key: Key, members: Vec<Vec<u8>>) -> Self {
        Self { key, members }
    }
}

pub struct SAddUseCase<R: ?Sized + SetRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + SetRepository> SAddUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: SAddInput) -> Result<usize, DomainError> {
        let span = info_span!("audit.sadd_use_case", key = %input.key.as_str(), members_len = input.members.len());
        let _enter = span.enter();
        let result = self.repository.sadd(&input.key, input.members).await;
        event!(Level::INFO, op = "sadd", key = %input.key.as_str(), success = result.is_ok());
        if let Err(e) = &result {
            event!(Level::WARN, error = %e.to_string());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockSetRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_sadd_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let members = vec![b"member1".to_vec(), b"member2".to_vec()];
        let input = SAddInput::new(key.clone(), members.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.members, members);
    }

    #[tokio::test]
    async fn test_sadd_use_case_new() {
        let mock_repo = Arc::new(MockSetRepository::new());
        let _use_case = SAddUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_sadd_use_case_execute_success() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let members = vec![b"member1".to_vec(), b"member2".to_vec()];

        mock_repo
            .expect_sadd()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = SAddUseCase::new(Arc::new(mock_repo));
        let input = SAddInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_sadd_use_case_execute_single_member() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let members = vec![b"single_member".to_vec()];

        mock_repo
            .expect_sadd()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = SAddUseCase::new(Arc::new(mock_repo));
        let input = SAddInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_sadd_use_case_execute_duplicate_members() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("duplicate_key".to_string()).unwrap();
        let members = vec![b"member1".to_vec(), b"member1".to_vec()];

        mock_repo
            .expect_sadd()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = SAddUseCase::new(Arc::new(mock_repo));
        let input = SAddInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_sadd_use_case_execute_empty_members() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let members: Vec<Vec<u8>> = vec![];

        mock_repo
            .expect_sadd()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = SAddUseCase::new(Arc::new(mock_repo));
        let input = SAddInput::new(key, members);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_sadd_use_case_execute_error() {
        let mut mock_repo = MockSetRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let members = vec![b"error_member".to_vec()];

        mock_repo
            .expect_sadd()
            .with(eq(key.clone()), eq(members.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = SAddUseCase::new(Arc::new(mock_repo));
        let input = SAddInput::new(key, members);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
