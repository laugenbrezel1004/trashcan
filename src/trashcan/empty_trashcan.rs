use std::fs;
use owo_colors::OwoColorize;
use crate::trashcan::core::Trashcan;
impl Trashcan {
    pub fn empty_trash(&self, interactive: bool) -> Result<(), String> {
        if interactive {
            let answer = dialoguer::Confirm::new()
                .with_prompt("Are you sure you want to empty the trashcan?")
                .interact()
                .map_err(|e| format!("Failed to get user input: {e}"))?;

            if !answer {
                println!("{}", "Operation cancelled".yellow());
                return Ok(());
            }
        }

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
                    .map_err(|e| format!("Failed to remove directory {path:?}: {e}").red().bold().to_string())?;
            } else {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed to remove file {path:?}: {e}").red().bold().to_string())?;
            }
        }

        println!("{}", "✓ Trashcan emptied successfully".green());
        Ok(())
    }
}