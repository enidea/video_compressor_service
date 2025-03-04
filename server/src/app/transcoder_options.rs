pub struct TranscoderOptions {
    pub bitrate: Option<usize>,
    pub preset: Option<String>,
}

impl TranscoderOptions {
    pub fn new(bitrate: Option<usize>, preset: Option<String>) -> Self {
        Self { bitrate, preset }
    }
}
