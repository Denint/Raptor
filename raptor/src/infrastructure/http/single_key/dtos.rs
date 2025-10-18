use crate::domain::value_objects::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ArraySetRequest {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayGetRequest {
    pub indices: Vec<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayGetResponse {
    pub values: Vec<Option<Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayAppendRequest {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayAppendResponse {
    pub new_length: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ArraySliceRequest {
    pub start: usize,
    #[serde(default)]
    pub end: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct ArraySliceResponse {
    pub values: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayUpdateRequest {
    pub updates: Vec<ArrayUpdateEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayUpdateEntry {
    pub index: usize,
    pub value: Value,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayUpdateResponse {
    pub updated_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ArrayLengthResponse {
    pub length: Option<usize>,
}
