use crate::domain::errors::DomainError;
use serde::{
    Deserialize, Serialize,
    de::{self, Deserializer, MapAccess, Visitor},
    ser::{SerializeStruct, Serializer},
};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(Vec<u8>),
    Integer(i64),
    Hash(HashMap<String, Vec<u8>>),
    List(Vec<Vec<u8>>),
    Set(HashSet<Vec<u8>>),
    SortedSet(Vec<(f64, Vec<u8>)>),
    Array(Vec<Value>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "String")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::Integer(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "Integer")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::Hash(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "Hash")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::List(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "List")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::Set(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "Set")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::SortedSet(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "SortedSet")?;
                state.serialize_field("value", v)?;
                state.end()
            }
            Value::Array(v) => {
                let mut state = serializer.serialize_struct("Value", 2)?;
                state.serialize_field("type", "Array")?;
                state.serialize_field("value", v)?;
                state.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Value,
        }

        struct ValueVisitor;

        impl<'de> Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Value")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_str: Option<String> = None;
                let mut value: Option<serde_json::Value> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_str.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            type_str = Some(map.next_value()?);
                        }
                        Field::Value => {
                            if value.is_some() {
                                return Err(de::Error::duplicate_field("value"));
                            }
                            value = Some(map.next_value()?);
                        }
                    }
                }

                let type_str = type_str.ok_or_else(|| de::Error::missing_field("type"))?;
                let value = value.ok_or_else(|| de::Error::missing_field("value"))?;

                match type_str.as_str() {
                    "String" => serde_json::from_value(value)
                        .map(Value::String)
                        .map_err(de::Error::custom),
                    "Integer" => serde_json::from_value(value)
                        .map(Value::Integer)
                        .map_err(de::Error::custom),
                    "Hash" => serde_json::from_value(value)
                        .map(Value::Hash)
                        .map_err(de::Error::custom),
                    "List" => serde_json::from_value(value)
                        .map(Value::List)
                        .map_err(de::Error::custom),
                    "Set" => serde_json::from_value(value)
                        .map(Value::Set)
                        .map_err(de::Error::custom),
                    "SortedSet" => serde_json::from_value(value)
                        .map(Value::SortedSet)
                        .map_err(de::Error::custom),
                    "Array" => serde_json::from_value(value)
                        .map(Value::Array)
                        .map_err(de::Error::custom),
                    _ => Err(de::Error::unknown_variant(
                        &type_str,
                        &[
                            "String",
                            "Integer",
                            "Hash",
                            "List",
                            "Set",
                            "SortedSet",
                            "Array",
                        ],
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &["type", "value"];
        deserializer.deserialize_struct("Value", FIELDS, ValueVisitor)
    }
}

impl Value {
    pub fn as_bytes(&self) -> Result<Vec<u8>, DomainError> {
        match self {
            Value::String(bytes) => Ok(bytes.clone()),
            Value::Integer(int) => Ok(int.to_string().into_bytes()),
            Value::Hash(hash) => bincode::serialize(hash).map_err(|_| DomainError::Unexpected),
            Value::List(list) => bincode::serialize(list).map_err(|_| DomainError::Unexpected),
            Value::Set(set) => bincode::serialize(set).map_err(|_| DomainError::Unexpected),
            Value::SortedSet(sorted_set) => {
                bincode::serialize(sorted_set).map_err(|_| DomainError::Unexpected)
            }
            Value::Array(array) => bincode::serialize(array).map_err(|_| DomainError::Unexpected),
        }
    }

    pub fn into_vec(self) -> Result<Vec<u8>, DomainError> {
        match self {
            Value::String(bytes) => Ok(bytes),
            Value::Integer(int) => Ok(int.to_string().into_bytes()),
            Value::Hash(hash) => bincode::serialize(&hash).map_err(|_| DomainError::Unexpected),
            Value::List(list) => bincode::serialize(&list).map_err(|_| DomainError::Unexpected),
            Value::Set(set) => bincode::serialize(&set).map_err(|_| DomainError::Unexpected),
            Value::SortedSet(sorted_set) => {
                bincode::serialize(&sorted_set).map_err(|_| DomainError::Unexpected)
            }
            Value::Array(array) => bincode::serialize(&array).map_err(|_| DomainError::Unexpected),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_value_as_bytes_string() {
        let value = Value::String(b"hello".to_vec());
        assert_eq!(value.as_bytes().unwrap(), b"hello");
    }

    #[test]
    fn test_value_as_bytes_integer() {
        let value = Value::Integer(42);
        assert_eq!(value.as_bytes().unwrap(), b"42");
    }

    #[test]
    fn test_value_as_bytes_hash() {
        let mut hash = HashMap::new();
        hash.insert("key".to_string(), b"value".to_vec());
        let value = Value::Hash(hash.clone());
        let expected = bincode::serialize(&hash).unwrap();
        assert_eq!(value.as_bytes().unwrap(), expected);
    }

    #[test]
    fn test_value_into_vec_string() {
        let value = Value::String(b"world".to_vec());
        assert_eq!(value.into_vec().unwrap(), b"world");
    }

    #[test]
    fn test_value_into_vec_integer() {
        let value = Value::Integer(123);
        assert_eq!(value.into_vec().unwrap(), b"123");
    }

    #[test]
    fn test_value_into_vec_hash() {
        let mut hash = HashMap::new();
        hash.insert("key".to_string(), b"value".to_vec());
        let value = Value::Hash(hash.clone());
        let expected = bincode::serialize(&hash).unwrap();
        assert_eq!(value.into_vec().unwrap(), expected);
    }

    #[test]
    fn test_value_serialize_string() {
        let value = Value::String(b"test".to_vec());
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"String","value":[116,101,115,116]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_serialize_integer() {
        let value = Value::Integer(42);
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"Integer","value":42}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_serialize_hash() {
        let mut hash = HashMap::new();
        hash.insert("field".to_string(), b"value".to_vec());
        let value = Value::Hash(hash);
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"Hash","value":{"field":[118,97,108,117,101]}}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_serialize_list() {
        let list = vec![b"item1".to_vec(), b"item2".to_vec()];
        let value = Value::List(list);
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"List","value":[[105,116,101,109,49],[105,116,101,109,50]]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_serialize_set() {
        let mut set = HashSet::new();
        set.insert(b"member1".to_vec());
        set.insert(b"member2".to_vec());
        let value = Value::Set(set);
        let serialized = serde_json::to_string(&value).unwrap();
        assert!(serialized.contains(r#""type":"Set""#));
        assert!(serialized.contains(r#""value""#));
    }

    #[test]
    fn test_value_serialize_sorted_set() {
        let sorted_set = vec![(1.5, b"member1".to_vec()), (2.5, b"member2".to_vec())];
        let value = Value::SortedSet(sorted_set);
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"SortedSet","value":[[1.5,[109,101,109,98,101,114,49]],[2.5,[109,101,109,98,101,114,50]]]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_serialize_array() {
        let array = vec![Value::String(b"str".to_vec()), Value::Integer(10)];
        let value = Value::Array(array);
        let serialized = serde_json::to_string(&value).unwrap();
        let expected = r#"{"type":"Array","value":[{"type":"String","value":[115,116,114]},{"type":"Integer","value":10}]}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_value_deserialize_string() {
        let json = r#"{"type":"String","value":[104,101,108,108,111]}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::String(bytes) => assert_eq!(bytes, b"hello"),
            _ => panic!("Expected String variant"),
        }
    }

    #[test]
    fn test_value_deserialize_integer() {
        let json = r#"{"type":"Integer","value":99}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::Integer(num) => assert_eq!(num, 99),
            _ => panic!("Expected Integer variant"),
        }
    }

    #[test]
    fn test_value_deserialize_hash() {
        let json = r#"{"type":"Hash","value":{"key":[118,97,108]}}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::Hash(hash) => {
                assert_eq!(hash.get("key"), Some(&b"val".to_vec()));
            }
            _ => panic!("Expected Hash variant"),
        }
    }

    #[test]
    fn test_value_deserialize_list() {
        let json = r#"{"type":"List","value":[[97],[98]]}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::List(list) => {
                assert_eq!(list, vec![b"a".to_vec(), b"b".to_vec()]);
            }
            _ => panic!("Expected List variant"),
        }
    }

    #[test]
    fn test_value_deserialize_set() {
        let json = r#"{"type":"Set","value":[[120],[121]]}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::Set(set) => {
                #[allow(clippy::unnecessary_to_owned)]
                {
                    assert!(set.contains(&b"x".to_vec()));
                    assert!(set.contains(&b"y".to_vec()));
                }
            }
            _ => panic!("Expected Set variant"),
        }
    }

    #[test]
    fn test_value_deserialize_sorted_set() {
        let json = r#"{"type":"SortedSet","value":[[1.0,[122]],[2.0,[119]]]}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::SortedSet(sorted_set) => {
                assert_eq!(sorted_set, vec![(1.0, b"z".to_vec()), (2.0, b"w".to_vec())]);
            }
            _ => panic!("Expected SortedSet variant"),
        }
    }

    #[test]
    fn test_value_deserialize_array() {
        let json = r#"{"type":"Array","value":[{"type":"Integer","value":5}]}"#;
        let value: Value = serde_json::from_str(json).unwrap();
        match value {
            Value::Array(array) => {
                assert_eq!(array.len(), 1);
                match &array[0] {
                    Value::Integer(num) => assert_eq!(*num, 5),
                    _ => panic!("Expected Integer in array"),
                }
            }
            _ => panic!("Expected Array variant"),
        }
    }

    #[test]
    fn test_value_deserialize_unknown_type() {
        let json = r#"{"type":"Unknown","value":null}"#;
        let result: Result<Value, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_deserialize_missing_type() {
        let json = r#"{"value":"test"}"#;
        let result: Result<Value, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_deserialize_missing_value() {
        let json = r#"{"type":"String"}"#;
        let result: Result<Value, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_deserialize_duplicate_type() {
        let json = r#"{"type":"String","type":"Integer","value":[116,101,115,116]}"#;
        let result: Result<Value, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_deserialize_duplicate_value() {
        let json = r#"{"type":"String","value":[116,101,115,116],"value":[97,98,99]}"#;
        let result: Result<Value, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_value_as_bytes_list() {
        let list = vec![b"item1".to_vec(), b"item2".to_vec()];
        let value = Value::List(list.clone());
        let expected = bincode::serialize(&list).unwrap();
        assert_eq!(value.as_bytes().unwrap(), expected);
    }

    #[test]
    fn test_value_as_bytes_set() {
        let mut set = HashSet::new();
        set.insert(b"member1".to_vec());
        set.insert(b"member2".to_vec());
        let value = Value::Set(set.clone());
        let expected = bincode::serialize(&set).unwrap();
        assert_eq!(value.as_bytes().unwrap(), expected);
    }

    #[test]
    fn test_value_as_bytes_sorted_set() {
        let sorted_set = vec![(1.5, b"member1".to_vec()), (2.5, b"member2".to_vec())];
        let value = Value::SortedSet(sorted_set.clone());
        let expected = bincode::serialize(&sorted_set).unwrap();
        assert_eq!(value.as_bytes().unwrap(), expected);
    }

    #[test]
    fn test_value_as_bytes_array() {
        let array = vec![Value::String(b"str".to_vec()), Value::Integer(10)];
        let value = Value::Array(array.clone());
        let expected = bincode::serialize(&array).unwrap();
        assert_eq!(value.as_bytes().unwrap(), expected);
    }

    #[test]
    fn test_value_into_vec_list() {
        let list = vec![b"item1".to_vec(), b"item2".to_vec()];
        let value = Value::List(list.clone());
        let expected = bincode::serialize(&list).unwrap();
        assert_eq!(value.into_vec().unwrap(), expected);
    }

    #[test]
    fn test_value_into_vec_set() {
        let mut set = HashSet::new();
        set.insert(b"member1".to_vec());
        set.insert(b"member2".to_vec());
        let value = Value::Set(set.clone());
        let expected = bincode::serialize(&set).unwrap();
        assert_eq!(value.into_vec().unwrap(), expected);
    }

    #[test]
    fn test_value_into_vec_sorted_set() {
        let sorted_set = vec![(1.5, b"member1".to_vec()), (2.5, b"member2".to_vec())];
        let value = Value::SortedSet(sorted_set.clone());
        let expected = bincode::serialize(&sorted_set).unwrap();
        assert_eq!(value.into_vec().unwrap(), expected);
    }

    #[test]
    fn test_value_into_vec_array() {
        let array = vec![Value::String(b"str".to_vec()), Value::Integer(10)];
        let value = Value::Array(array.clone());
        let expected = bincode::serialize(&array).unwrap();
        assert_eq!(value.into_vec().unwrap(), expected);
    }

    #[test]
    fn test_value_empty_string() {
        let value = Value::String(vec![]);
        assert_eq!(value.as_bytes().unwrap(), Vec::<u8>::new());
        assert_eq!(value.into_vec().unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_value_zero_integer() {
        let value = Value::Integer(0);
        assert_eq!(value.as_bytes().unwrap(), b"0");
        assert_eq!(value.into_vec().unwrap(), b"0");
    }

    #[test]
    fn test_value_negative_integer() {
        let value = Value::Integer(-42);
        assert_eq!(value.as_bytes().unwrap(), b"-42");
        assert_eq!(value.into_vec().unwrap(), b"-42");
    }

    #[test]
    fn test_value_empty_collections() {
        let empty_hash = Value::Hash(HashMap::new());
        let empty_list = Value::List(Vec::new());
        let empty_set = Value::Set(HashSet::new());
        let empty_sorted_set = Value::SortedSet(Vec::new());
        let empty_array = Value::Array(Vec::new());

        assert!(serde_json::to_string(&empty_hash).is_ok());
        assert!(serde_json::to_string(&empty_list).is_ok());
        assert!(serde_json::to_string(&empty_set).is_ok());
        assert!(serde_json::to_string(&empty_sorted_set).is_ok());
        assert!(serde_json::to_string(&empty_array).is_ok());
    }

    #[test]
    fn test_value_equality() {
        let value1 = Value::String(b"test".to_vec());
        let value2 = Value::String(b"test".to_vec());
        let value3 = Value::String(b"different".to_vec());

        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
        assert_ne!(value1, Value::Integer(42));
    }
}
