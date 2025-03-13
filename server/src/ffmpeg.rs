mod options;

use options::Options;
pub use options::OptionsBuilder;

use std::path::Path;

pub fn convert(
    input_file_path: &Path,
    output_file_path: &Path,
    options: Options,
) -> anyhow::Result<()> {
    Ok(())
}
