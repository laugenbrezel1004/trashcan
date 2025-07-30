use std::fs;
use std::path::Path;
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;

pub struct Trashcan {
    trashcan_path: Path;
}
impl Trashcan {
    /// Initialize the trashcan with proper directory structure
    pub fn initialize() -> Result<Self, String> {
        let home_dir = get_user_by_uid(get_current_uid())
            .ok_or("Failed to get current user")?
            .home_dir()
            .to_path_buf();

        let trash_path = home_dir.join(".local/share/trashcan");


        fs::create_dir_all(trash_path)
            .map_err(|e| format!("Failed to create trashcan info directory: {e}"))?;

        Ok(Self {
            trashcan_path
        })
    }
}
