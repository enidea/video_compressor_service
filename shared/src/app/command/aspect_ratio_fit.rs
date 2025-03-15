use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, EnumIter, EnumString, Serialize, Deserialize, Default, Display)]
pub enum AspectRatioFit {
    #[default]
    ForceFit,
    BlackPadding,
}
