use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HSetResponse {
    pub is_new_entry: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HGetResponse {
    pub field: String,
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HDelResponse {
    pub deleted_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct HExistsResponse {
    pub exists: bool,
}

#[derive(Serialize, Deserialize)]
pub struct HKeysResponse {
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HValsResponse {
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct HLenResponse {
    pub len: usize,
}
