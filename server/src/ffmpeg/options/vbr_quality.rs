#[derive(Debug, Clone, Copy)]
pub struct VbrQuality {
    value: u8,
}

impl VbrQuality {
    pub fn new(value: u8) -> anyhow::Result<Self> {
        if !(0..=9).contains(&value) {
            anyhow::bail!("VBR quality value must be between 0 and 9");
        } else {
            Ok(Self { value })
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
