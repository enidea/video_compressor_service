#[derive(Debug, Clone, Copy)]
pub struct ClipRange {
    pub start: u32,
    pub end: u32,
}

impl ClipRange {
    pub fn new(start: u32, end: u32) -> anyhow::Result<Self> {
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

    pub fn formatted_start(&self) -> String {
        format!(
            "{}",
            chrono::NaiveTime::from_num_seconds_from_midnight_opt(self.start, 0)
                .unwrap()
                .format("%H:%M:%S")
        )
    }

    pub fn formatted_end(&self) -> String {
        format!(
            "{}",
            chrono::NaiveTime::from_num_seconds_from_midnight_opt(self.end, 0)
                .unwrap()
                .format("%H:%M:%S")
        )
    }
}
