use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ClipRange {
    start: u32,
    end: u32,
}

impl ClipRange {
    pub fn new(start: u32, end: u32) -> anyhow::Result<Self> {
        println!("start: {}, end: {}", start, end);
        if start >= end {
            anyhow::bail!("Clip range start must be less than end");
        }

        Ok(Self { start, end })
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn end(&self) -> u32 {
        self.end
    }
}
