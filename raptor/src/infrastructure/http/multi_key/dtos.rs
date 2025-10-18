use crate::application::dtos::value_dto::ValueDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MSetPair {
    pub key: String,
    pub value: ValueDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MSetRequest {
    pub pairs: Vec<MSetPair>,
}

#[derive(Serialize, Deserialize)]
pub struct MGetRequest {
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MGetResponse {
    pub values: Vec<Option<ValueDto>>,
}

#[derive(Serialize, Deserialize)]
pub struct MDelRequest {
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MDelResponse {
    pub values: Vec<Option<ValueDto>>,
}
