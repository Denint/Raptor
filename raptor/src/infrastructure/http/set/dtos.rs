use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SAddRequest {
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SAddResponse {
    pub added_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SMembersResponse {
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SRemRequest {
    pub members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SRemResponse {
    pub removed_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SIsMemberRequest {
    pub member: String,
}

#[derive(Serialize, Deserialize)]
pub struct SIsMemberResponse {
    pub is_member: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SCardResponse {
    pub card: usize,
}
