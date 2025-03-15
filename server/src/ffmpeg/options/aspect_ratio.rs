use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
#[display("{}:{}", width, height)]
pub struct AspectRatio {
    width: u32,
    height: u32,
}

impl AspectRatio {
    pub fn new(width: u32, height: u32) -> anyhow::Result<Self> {
        if width == 0 || height == 0 {
            anyhow::bail!("Aspect ratio width and height must be greater than 0");
        }

        let gcd = num::integer::gcd(width, height);

        Ok(Self {
            width: width / gcd,
            height: height / gcd,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
