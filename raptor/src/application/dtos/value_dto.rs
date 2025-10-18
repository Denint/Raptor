use crate::domain::value_objects::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValueDto(pub Value);

impl From<Value> for ValueDto {
    fn from(value: Value) -> Self {
        ValueDto(value)
    }
}

impl From<ValueDto> for Value {
    fn from(value_dto: ValueDto) -> Self {
        value_dto.0
    }
}
