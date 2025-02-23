use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Primitive, Serialize, Deserialize)]
pub enum Status {
    Ok = 200,
    BadRequest = 400,
    InternalServerError = 500,
}

impl Status {
    pub fn try_from_u16(value: u16) -> anyhow::Result<Self> {
        FromPrimitive::from_u16(value)
            .ok_or_else(|| anyhow::anyhow!("Invalid status code: {}", value))
    }
}
