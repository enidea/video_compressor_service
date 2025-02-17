pub struct Json {
    pub data: serde_json::Value,
}

impl Json {
    pub const HEADER_SIZE_BYTES: usize = 2;

    pub fn new(data: serde_json::Value) -> Self {
        Self { data }
    }
}
