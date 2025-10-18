use crate::domain::value_objects::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredValue {
    pub value: Value,
    pub expires_at: Option<u64>,
    pub last_accessed: u64,
}

impl std::fmt::Display for StoredValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl From<String> for StoredValue {
    fn from(s: String) -> Self {
        StoredValue::new(Value::String(s.into_bytes()))
    }
}

impl From<&str> for StoredValue {
    fn from(s: &str) -> Self {
        StoredValue::new(Value::String(s.to_string().into_bytes()))
    }
}

impl StoredValue {
    pub fn new(value: Value) -> Self {
        let now = get_unix_timestamp();
        Self {
            value,
            expires_at: None,
            last_accessed: now,
        }
    }

    pub fn with_ttl(value: Value, ttl_seconds: u64) -> Self {
        let now = get_unix_timestamp();
        let expires_at = now + (ttl_seconds * 1000);
        Self {
            value,
            expires_at: Some(expires_at),
            last_accessed: now,
        }
    }

    pub fn with_ttl_ms(value: Value, ttl_ms: u64) -> Self {
        let now = get_unix_timestamp();
        let expires_at = now + ttl_ms;
        Self {
            value,
            expires_at: Some(expires_at),
            last_accessed: now,
        }
    }

    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            get_unix_timestamp() > expires_at
        } else {
            false
        }
    }

    pub fn ttl_remaining(&self) -> Option<u64> {
        let now = get_unix_timestamp();
        self.expires_at
            .map(|expires_at| expires_at.saturating_sub(now))
    }

    pub fn update_access(&mut self) {
        self.last_accessed = get_unix_timestamp();
    }
}

pub fn get_unix_timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::ZERO)
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::value::Value;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_stored_value_new() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::new(value.clone());

        match stored.value {
            Value::String(bytes) => assert_eq!(bytes, b"test"),
            _ => panic!("Expected String value"),
        }
        assert!(stored.expires_at.is_none());
        assert!(stored.last_accessed > 0);
    }

    #[test]
    fn test_stored_value_with_ttl() {
        let value = Value::Integer(42);
        let stored = StoredValue::with_ttl(value.clone(), 10);

        match stored.value {
            Value::Integer(num) => assert_eq!(num, 42),
            _ => panic!("Expected Integer value"),
        }
        assert!(stored.expires_at.is_some());
        assert!(stored.last_accessed > 0);

        let now = get_unix_timestamp();
        let expected_expires = now + (10 * 1000);
        assert!(stored.expires_at.unwrap() >= expected_expires - 10);
        assert!(stored.expires_at.unwrap() <= expected_expires + 10);
    }

    #[test]
    fn test_stored_value_with_ttl_ms() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl_ms(value.clone(), 5000);

        match stored.value {
            Value::String(bytes) => assert_eq!(bytes, b"test"),
            _ => panic!("Expected String value"),
        }
        assert!(stored.expires_at.is_some());
        assert!(stored.last_accessed > 0);

        let now = get_unix_timestamp();
        let expected_expires = now + 5000;
        assert!(stored.expires_at.unwrap() >= expected_expires - 10);
        assert!(stored.expires_at.unwrap() <= expected_expires + 10);
    }

    #[test]
    fn test_stored_value_is_expired_no_ttl() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::new(value);
        assert!(!stored.is_expired());
    }

    #[test]
    fn test_stored_value_is_expired_future() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl(value, 3600);
        assert!(!stored.is_expired());
    }

    #[test]
    fn test_stored_value_is_expired_past() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl_ms(value, 1);
        thread::sleep(Duration::from_millis(2));
        assert!(stored.is_expired());
    }

    #[test]
    fn test_stored_value_ttl_remaining_no_ttl() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::new(value);
        assert_eq!(stored.ttl_remaining(), None);
    }

    #[test]
    fn test_stored_value_ttl_remaining_future() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl(value, 10);
        let remaining = stored.ttl_remaining().unwrap();
        assert!(remaining > 0);
        assert!(remaining <= 10000);
    }

    #[test]
    fn test_stored_value_ttl_remaining_expired() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl_ms(value, 1);
        thread::sleep(Duration::from_millis(2));
        let remaining = stored.ttl_remaining().unwrap();
        assert_eq!(remaining, 0);
    }

    #[test]
    fn test_stored_value_update_access() {
        let value = Value::String(b"test".to_vec());
        let mut stored = StoredValue::new(value);
        let original_access = stored.last_accessed;

        thread::sleep(Duration::from_millis(1));
        stored.update_access();

        assert!(stored.last_accessed >= original_access);
    }

    #[test]
    fn test_stored_value_display() {
        let value = Value::String(b"hello world".to_vec());
        let stored = StoredValue::new(value);
        let display_str = format!("{}", stored);
        assert!(display_str.contains("String"));
        assert!(display_str.contains("[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]"));
    }

    #[test]
    fn test_stored_value_from_string() {
        let stored: StoredValue = "test string".to_string().into();
        match stored.value {
            Value::String(bytes) => assert_eq!(bytes, b"test string"),
            _ => panic!("Expected String value"),
        }
        assert!(stored.expires_at.is_none());
    }

    #[test]
    fn test_stored_value_from_str() {
        let stored: StoredValue = "test str".into();
        match stored.value {
            Value::String(bytes) => assert_eq!(bytes, b"test str"),
            _ => panic!("Expected String value"),
        }
        assert!(stored.expires_at.is_none());
    }

    #[test]
    fn test_get_unix_timestamp() {
        let timestamp1 = get_unix_timestamp();
        thread::sleep(Duration::from_millis(1));
        let timestamp2 = get_unix_timestamp();

        assert!(timestamp2 >= timestamp1);
        assert!(timestamp1 > 0);
    }

    #[test]
    fn test_stored_value_serialization() {
        let value = Value::Integer(123);
        let stored = StoredValue::with_ttl(value, 60);

        let serialized = serde_json::to_string(&stored).unwrap();

        let deserialized: StoredValue = serde_json::from_str(&serialized).unwrap();

        match deserialized.value {
            Value::Integer(num) => assert_eq!(num, 123),
            _ => panic!("Expected Integer value"),
        }
        assert_eq!(deserialized.expires_at, stored.expires_at);
        assert_eq!(deserialized.last_accessed, stored.last_accessed);
    }

    #[test]
    fn test_stored_value_expired_ttl() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl_ms(value, 1);
        thread::sleep(Duration::from_millis(2));
        assert!(stored.is_expired());
    }

    #[test]
    fn test_stored_value_not_expired() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl(value, 3600);
        assert!(!stored.is_expired());
    }

    #[test]
    fn test_stored_value_zero_ttl() {
        let value = Value::String(b"test".to_vec());
        let stored = StoredValue::with_ttl_ms(value, 0);
        assert!(stored.expires_at.is_some());
        assert!(stored.expires_at.unwrap() <= get_unix_timestamp());
    }

    #[test]
    fn test_stored_value_access_time_update() {
        let value = Value::String(b"test".to_vec());
        let mut stored = StoredValue::new(value);
        let initial_access = stored.last_accessed;

        thread::sleep(Duration::from_millis(1));
        stored.update_access();

        assert!(stored.last_accessed > initial_access);
    }

    #[test]
    fn test_stored_value_different_value_types() {
        let test_cases = vec![
            Value::String(b"string".to_vec()),
            Value::Integer(42),
            Value::Hash(std::collections::HashMap::new()),
            Value::List(vec![]),
            Value::Set(std::collections::HashSet::new()),
            Value::SortedSet(vec![]),
            Value::Array(vec![]),
        ];

        for value in test_cases {
            let stored = StoredValue::new(value.clone());
            assert_eq!(stored.value, value);
            assert!(!stored.is_expired());
            assert_eq!(stored.ttl_remaining(), None);
        }
    }
}
