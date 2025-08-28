use crate::trashcan::core::Trashcan;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use owo_colors::OwoColorize;
use uuid::Uuid;

impl Trashcan {
    pub fn move_to_trash(&self, infile: &str) -> Result<(), String> {
        let src = Path::new(infile);
        if !src.exists() {
            return Err(format!("File '{}' does not exist", infile.red().bold()));
        }

        let uuid = Uuid::new_v4();
        let uuid = uuid.to_string()[0..4].to_string();
        let outfile = format!(
            "{}~{:?}",
            src.file_name().ok_or("Invalid filename")?.to_string_lossy(),
            uuid
        );

        let dest = self.trashcan_path.join(outfile);
        // TODO: does this work over filesystem boundaries
        fs::rename(src, &dest).map_err(|e| format!("Failed to move file to trash: {e}").red().bold().to_string())?;
        
        Ok(())
    }
}
