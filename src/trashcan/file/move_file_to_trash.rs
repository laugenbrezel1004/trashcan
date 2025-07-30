use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod test {
    pub fn move_to_trash(&self, file: &str) -> Result<(), String> {
        let src = Path::new(file);
        if !src.exists() {
            return Err(format!("File '{}' does not exist", file.red()));
        }

        //let uuid = Uuid::new_v4()
        //let uuid = Uuid::new_v7();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let dest_name = format!(
            "{}~{:?}",
            src.file_name().ok_or("Invalid filename")?.to_string_lossy(),
            timestamp
        );

        let dest = self.path.join(&dest_name);
        fs::rename(src, &dest).map_err(|e| format!("Failed to move file to trash: {e}"))?;

        // Create trashinfo file (compatible with FreeDesktop.org Trash spec)
        let info_content = format!(
            "[Trash Info]\nPath={}\nDeletionDate={}",
            src.to_string_lossy(),
            Local::now().format("%Y-%m-%dT%H:%M:%S")
        );

        fs::write(
            self.info_dir.join(format!("{:?}.trashinfo", timestamp)),
            info_content,
        )
        .map_err(|e| format!("Failed to create trash info: {e}"))?;

        Ok(())
    }
}
