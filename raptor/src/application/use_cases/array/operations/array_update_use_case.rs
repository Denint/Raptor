use crate::domain::{
    errors::DomainError,
    repositories::ArrayRepository,
    value_objects::{key::Key, value::Value},
};
use std::sync::Arc;

pub struct ArrayUpdateInput {
    pub key: Key,
    pub updates: Vec<(usize, Value)>,
}

impl ArrayUpdateInput {
    pub fn new(key: Key, updates: Vec<(usize, Value)>) -> Self {
        Self { key, updates }
    }
}

pub struct ArrayUpdateUseCase<R: ?Sized + ArrayRepository> {
    repository: Arc<R>,
}

impl<R: ?Sized + ArrayRepository> ArrayUpdateUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    #[inline]
    pub async fn execute(&self, input: ArrayUpdateInput) -> Result<usize, DomainError> {
        self.repository
            .array_update(&input.key, input.updates)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::repositories::MockArrayRepository;
    use mockall::predicate::*;

    #[tokio::test]
    async fn test_array_update_input_new() {
        let key = Key::new("test_key".to_string()).unwrap();
        let updates = vec![
            (0, Value::String(b"value".to_vec())),
            (2, Value::Integer(42)),
        ];
        let input = ArrayUpdateInput::new(key.clone(), updates.clone());
        assert_eq!(input.key, key);
        assert_eq!(input.updates, updates);
    }

    #[tokio::test]
    async fn test_array_update_use_case_new() {
        let mock_repo = Arc::new(MockArrayRepository::new());
        let _use_case = ArrayUpdateUseCase::new(mock_repo);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_success() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("test_key".to_string()).unwrap();
        let updates = vec![
            (0, Value::String(b"new_value0".to_vec())),
            (2, Value::Integer(42)),
        ];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Ok(2));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 2);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_single_update() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("single_key".to_string()).unwrap();
        let updates = vec![(1, Value::String(b"updated".to_vec()))];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_multiple_updates() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("multi_key".to_string()).unwrap();
        let updates = vec![
            (0, Value::String(b"first".to_vec())),
            (1, Value::Integer(1)),
            (2, Value::String(b"third".to_vec())),
        ];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Ok(3));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 3);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_partial_updates() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("partial_key".to_string()).unwrap();
        let updates = vec![
            (0, Value::String(b"valid".to_vec())),
            (10, Value::Integer(42)),
        ];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Ok(1));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 1);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_empty_updates() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("empty_key".to_string()).unwrap();
        let updates: Vec<(usize, Value)> = vec![];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Ok(0));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await.unwrap();
        assert_eq!(result, 0);
    }

    #[tokio::test]
    async fn test_array_update_use_case_execute_error() {
        let mut mock_repo = MockArrayRepository::new();
        let key = Key::new("error_key".to_string()).unwrap();
        let updates = vec![(0, Value::String(b"error_value".to_vec()))];

        mock_repo
            .expect_array_update()
            .with(eq(key.clone()), eq(updates.clone()))
            .times(1)
            .returning(|_, _| Err(DomainError::StorageError("test error".to_string())));

        let use_case = ArrayUpdateUseCase::new(Arc::new(mock_repo));
        let input = ArrayUpdateInput::new(key, updates);

        let result = use_case.execute(input).await;
        assert!(matches!(result, Err(DomainError::StorageError(_))));
    }
}
