// src/trashcan/core.rs
use std::fs;
use std::path::{Path, PathBuf};
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;

/// Main trashcan implementation
pub struct Trashcan {
    pub trashcan_path: PathBuf,
}

impl Trashcan {
    /// Initializes the trashcan directory structure
    pub fn initialize() -> Result<Self, String> {
        // the trashcan directory is directly in $HOME/.trashcan
        let home_dir = get_user_by_uid(get_current_uid())
            .ok_or("Failed to get current user")?
            .home_dir()
            .to_path_buf();

        // create trashcan directory if it os not there
        let trash_path = home_dir.join(".trashcan");

        fs::create_dir_all(&trash_path)
            .map_err(|e| format!("Failed to create trashcan directory: {e}"))?;

        Ok(Self {
            trashcan_path: trash_path,
        })
    }

    /// Permanently deletes a file (bypasses trashcan)
    /// TODO: Replace with the nuke function
    pub fn delete_permanently(&self, file: &str) -> Result<(), String> {
        let path = Path::new(file);
        if path.is_dir() {
            fs::remove_dir_all(path)
                .map_err(|e| format!("Failed to delete directory: {e}"))?;
        } else {
            fs::remove_file(path)
                .map_err(|e| format!("Failed to delete file: {e}"))?;
        }
        Ok(())
    }

}
