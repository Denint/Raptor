use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LPushResponse {
    pub new_length: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RPushResponse {
    pub new_length: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LPopResponse {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct RPopResponse {
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct LRangeResponse {
    pub values: Vec<String>,
}
