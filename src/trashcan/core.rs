use std::fs;
use std::path::PathBuf;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

/// Main trashcan implementation
pub struct Trashcan {
    pub trashcan_path: PathBuf,
}

impl Trashcan {
    /// Initializes the trashcan directory  directly in $HOME/.trashcan
    pub fn initialize() -> Result<Self, String> {
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
}
