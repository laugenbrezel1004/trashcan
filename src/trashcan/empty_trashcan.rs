use std::fs;
use owo_colors::OwoColorize;
use crate::trashcan::core::Trashcan;;
impl Trashcan {
    pub fn empty(&self) -> Result<(), String> {
        if !self.trashcan_path.exists() {
            return Err("Trashcan does not exist".red().bold().to_string());
        }

        // Remove files
        for entry in
            fs::read_dir(&self.trashcan_path).map_err(|e| format!("Failed to read trashcan: {e}").red().bold().to_string())?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {e}").red().bold().to_string())?;
            let path = entry.path();

            if path.is_dir() {
                fs::remove_dir_all(&path)
                    .map_err(|e| format!("Failed to remove directory {:?}: {e}", path).red().bold().to_string())?;
            } else {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed to remove file {:?}: {e}", path).red().bold().to_string())?;
            }
        }


        Ok(())
    }
}