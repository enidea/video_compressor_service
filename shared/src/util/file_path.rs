use std::path::{Path, PathBuf};

pub fn add_prefix_to_file_path(file_path: &Path, prefix: &str) -> anyhow::Result<PathBuf> {
    if !file_path.is_file() {
        return Err(anyhow::anyhow!("The file path is not a file"));
    }

    let dir = file_path.parent().unwrap_or_else(|| Path::new(""));
    let file_name = file_path
        .file_name()
        .ok_or(anyhow::anyhow!("The file path does not have a file name"))?;

    Ok(dir.join(format!("{}{}", prefix, file_name.to_string_lossy())))
}
