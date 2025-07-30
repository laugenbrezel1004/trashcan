use std::fs;
use std::path::Path;

pub fn nuke(&self, file: &str) -> Result<(), String> {
    let path = Path::new(file);
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| format!("Failed to delete directory: {e}"))?;
    } else {
        fs::remove_file(path).map_err(|e| format!("Failed to delete file: {e}"))?;
    }
    Ok(())
}