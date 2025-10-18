use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ZAddRequest {
    pub score: f64,
    pub member: String,
}

#[derive(Serialize, Deserialize)]
pub struct ZAddResponse {
    pub is_new_entry: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ZRangeRequest {
    pub start: isize,
    pub stop: isize,
}

#[derive(Serialize, Deserialize)]
pub struct ZRangeResponse {
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ZRemRequest {
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ZRemResponse {
    pub removed_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct ZScoreResponse {
    pub score: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct ZCardResponse {
    pub card: usize,
}
