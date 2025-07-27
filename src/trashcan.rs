use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;
use uuid::Uuid;

/// Represents the trashcan directory and its operations.
pub struct Trashcan {
    pub trashcan_homedirectory_location: PathBuf,
}
pub fn initialize_trashcan() -> Result<Trashcan, String> {
    let home_dir = get_user_by_uid(get_current_uid())
        .ok_or("Failed to get current user")?
        .home_dir()
        .to_path_buf();
    let trashcan_homedirectory_location = home_dir.join(".local/share/trashcan");

    let trashcan = Trashcan {
        trashcan_homedirectory_location,
    };
    trashcan.create_trashcan_directory()?;
    Ok(trashcan)
}

impl Trashcan {
    /// Initializes the trashcan directory at `~/.local/share/trashcan`.

    /// Creates the trashcan directory if it doesn't exist.
    pub fn create_trashcan_directory(&self) -> Result<(), String> {
        fs::create_dir_all(&self.trashcan_homedirectory_location)
            .map_err(|e| format!("failed to create trashcan directory: {}", e))?;
        Ok(())
    }

    /// Permanently deletes the entire trashcan directory and recreates it.
    pub fn nuke_trashcan_directory(&self) -> Result<(), String> {
        if self.trashcan_homedirectory_location.exists() {
            fs::remove_dir_all(&self.trashcan_homedirectory_location)
                .map_err(|e| format!("cannot clean trashcan directory: {}", e))?;
            self.create_trashcan_directory()?;
        }
        Ok(())
    }

    /// Displays the contents of the trashcan with file details.
    pub fn show_trashcan(&self) -> Result<(), String> {
        let entries = fs::read_dir(&self.trashcan_homedirectory_location)
            .map_err(|e| format!("cannot read trashcan directory: {}", e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("cannot read directory entry: {}", e))?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            let metadata = entry.metadata().map_err(|e| format!("cannot get metadata: {}", e))?;
            let size = metadata.len();
            let file_type = if metadata.is_dir() { "dir" } else { "file" };
            println!("{} ({}: {} bytes)", file_name, file_type, size);
        }
        Ok(())
    }

    /// Moves a file to the trashcan with a unique UUID.
    pub fn move_file_to_trashcan(&self, file: &str) -> Result<(), String> {
        let path = Path::new(file);
        if !path.exists() {
            return Err(format!("file '{}' does not exist", file));
        }

        let uuid = Uuid::new_v4().to_string();
        let dest = self.trashcan_homedirectory_location.join(&uuid);
        fs::rename(path, &dest).map_err(|e| format!("failed to move file '{}': {}", file, e))?;
        Ok(())
    }

    /// Permanently deletes a file.
    pub fn nuke_file(&self, file: &str) -> Result<(), String> {
        let path = Path::new(file);
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("cannot remove directory '{}': {}", file, e))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("cannot remove file '{}': {}", file, e))?;
        }
        Ok(())
    }

    /// Restores the most recently deleted file from the trashcan.
    pub fn restore_last_file(&self) -> Result<(), String> {
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.trashcan_homedirectory_location)
            .map_err(|e| format!("cannot read trashcan directory: {}", e))?
            .filter_map(Result::ok)
            .collect();

        if entries.is_empty() {
            return Err("trashcan is empty".to_string());
        }

        // Sort by modification time to get the most recent file
        entries.sort_by(|a, b| {
            let a_time = a.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            let b_time = b.metadata().and_then(|m| m.modified()).unwrap_or(std::time::SystemTime::UNIX_EPOCH);
            b_time.cmp(&a_time) // Reverse order to get most recent first
        });

        let latest = entries.first().unwrap();
        let file_name = latest.file_name().to_string_lossy().to_string();
        let dest = PathBuf::from(&file_name); // Restore to original name or current directory
        fs::rename(latest.path(), dest)
            .map_err(|e| format!("cannot restore file '{}': {}", file_name, e))?;
        println!("restored '{}'", file_name);
        Ok(())
    }
}