use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterResponse {
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionCounterResponse {
    pub value: Option<i64>,
}
