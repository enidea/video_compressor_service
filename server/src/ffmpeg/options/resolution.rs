use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("{}:{}", width, height)]
pub struct Resolution {
    width: u32,
    height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> anyhow::Result<Self> {
        if width == 0 || height == 0 {
            anyhow::bail!("Resolution width and height must be greater than 0");
        }

        Ok(Self { width, height })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
