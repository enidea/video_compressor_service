use derive_more::Display;

#[derive(Debug, Clone, Copy, Display)]
pub enum Preset {
    #[display("medium")]
    Medium,
    #[display("slower")]
    Slower,
}
