use derive_more::Display;

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub enum Preset {
    #[display("medium")]
    Medium,
    #[display("slower")]
    Slower,
}
