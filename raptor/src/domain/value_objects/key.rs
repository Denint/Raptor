use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct Key(String);

impl Key {
    #[inline]
    pub fn new(key: String) -> Result<Self, DomainError> {
        if key.is_empty() {
            return Err(DomainError::EmptyKey);
        }
        Ok(Self(key))
    }

    #[inline]
    pub fn from_static(key: &'static str) -> Self {
        Self(key.to_string())
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for Key {
    fn from(s: String) -> Self {
        Key(s)
    }
}

impl From<&str> for Key {
    fn from(s: &str) -> Self {
        Key(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::errors::DomainError;

    #[test]
    fn test_key_new_valid() {
        let key = Key::new("valid_key".to_string());
        assert!(key.is_ok());
        assert_eq!(key.unwrap().as_str(), "valid_key");
    }

    #[test]
    fn test_key_new_empty() {
        let key = Key::new("".to_string());
        assert_eq!(key, Err(DomainError::EmptyKey));
    }

    #[test]
    fn test_key_from_static() {
        let key = Key::from_static("static_key");
        assert_eq!(key.as_str(), "static_key");
    }

    #[test]
    fn test_key_as_str() {
        let key = Key::new("test_key".to_string()).unwrap();
        assert_eq!(key.as_str(), "test_key");
    }

    #[test]
    fn test_key_display() {
        let key = Key::new("display_key".to_string()).unwrap();
        assert_eq!(format!("{}", key), "display_key");
    }

    #[test]
    fn test_key_from_string() {
        let key: Key = "from_string".to_string().into();
        assert_eq!(key.as_str(), "from_string");
    }

    #[test]
    fn test_key_from_str() {
        let key: Key = "from_str".into();
        assert_eq!(key.as_str(), "from_str");
    }

    #[test]
    fn test_key_equality() {
        let key1 = Key::new("same_key".to_string()).unwrap();
        let key2 = Key::new("same_key".to_string()).unwrap();
        let key3 = Key::new("different_key".to_string()).unwrap();

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_key_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        let key1 = Key::new("hash_key".to_string()).unwrap();
        let key2 = Key::new("hash_key".to_string()).unwrap();

        set.insert(key1);
        assert!(!set.insert(key2));
    }

    #[test]
    fn test_key_special_characters() {
        let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
        let key = Key::new(special_chars.to_string()).unwrap();
        assert_eq!(key.as_str(), special_chars);
    }

    #[test]
    fn test_key_unicode_characters() {
        let unicode = "тест_key_🚀_🦀";
        let key = Key::new(unicode.to_string()).unwrap();
        assert_eq!(key.as_str(), unicode);
    }

    #[test]
    fn test_key_whitespace() {
        let with_spaces = "key with spaces";
        let key = Key::new(with_spaces.to_string()).unwrap();
        assert_eq!(key.as_str(), with_spaces);
    }

    #[test]
    fn test_key_numbers_only() {
        let numbers = "1234567890";
        let key = Key::new(numbers.to_string()).unwrap();
        assert_eq!(key.as_str(), numbers);
    }

    #[test]
    fn test_key_mixed_case() {
        let mixed = "KeyWithMixedCase123";
        let key = Key::new(mixed.to_string()).unwrap();
        assert_eq!(key.as_str(), mixed);
    }

    #[test]
    fn test_key_from_empty_string() {
        let key = Key::new("".to_string());
        assert!(matches!(key, Err(DomainError::EmptyKey)));
    }

    #[test]
    fn test_key_from_whitespace_only() {
        let key = Key::new("   ".to_string());
        assert!(key.is_ok());
        assert_eq!(key.unwrap().as_str(), "   ");
    }

    #[test]
    fn test_key_clone_behavior() {
        let original = Key::new("test_key".to_string()).unwrap();
        let cloned = original.clone();
        assert_eq!(original, cloned);
        assert_eq!(original.as_str(), cloned.as_str());
    }

    #[test]
    fn test_key_debug_format() {
        let key = Key::new("debug_test".to_string()).unwrap();
        let debug_str = format!("{:?}", key);
        assert!(debug_str.contains("Key"));
        assert!(debug_str.contains("debug_test"));
    }

    #[test]
    fn test_key_hash_consistency() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let key1 = Key::new("consistent_key".to_string()).unwrap();
        let key2 = Key::new("consistent_key".to_string()).unwrap();

        map.insert(key1, "value1");
        assert_eq!(map.get(&key2), Some(&"value1"));
    }

    #[test]
    fn test_key_different_keys_not_equal() {
        let key1 = Key::new("key1".to_string()).unwrap();
        let key2 = Key::new("key2".to_string()).unwrap();
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_key_long_key() {
        let long_key = "a".repeat(1000);
        let key = Key::new(long_key.clone()).unwrap();
        assert_eq!(key.as_str(), long_key);
    }
}
