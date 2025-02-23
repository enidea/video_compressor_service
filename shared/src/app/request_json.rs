use serde::{Deserialize, Serialize};

use super::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestJson {
    pub command: Command,
}

impl RequestJson {
    pub fn new(command: Command) -> Self {
        Self { command }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
