#[derive(Debug, Clone, Copy)]
pub struct Crf {
    value: u8,
}

impl Default for Crf {
    fn default() -> Self {
        Self::new(23).unwrap()
    }
}

impl Crf {
    pub fn new(value: u8) -> anyhow::Result<Self> {
        if !(0..=51).contains(&value) {
            anyhow::bail!("CRF value must be between 0 and 51");
        } else {
            Ok(Self { value })
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
