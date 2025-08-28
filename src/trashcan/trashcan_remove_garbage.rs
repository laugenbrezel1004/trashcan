use crate::trashcan::core::Trashcan;
use human_bytes::human_bytes;
use humansize::ToF64;
use owo_colors::OwoColorize;
use std::fs;
impl Trashcan {
    pub fn remove_garbage(&self, interactive: bool, verbose: bool) -> Result<(), String> {
        let answer = dialoguer::Confirm::new()
            .with_prompt("Are you sure you want to empty the trashcan?")
            .interact()
            .map_err(|e| format!("Failed to get user input: {e}"))?;

        if !answer {
            println!("{}", "Operation cancelled".yellow());
            return Ok(());
        }

        if !self.trashcan_path.exists() {
            return Err("Trashcan does not exist".red().bold().to_string());
        }

        let mut total_space_saved: f64 = 0.0;
        // Remove files
        for entry in fs::read_dir(&self.trashcan_path).map_err(|e| {
            format!("Failed to read trashcan: {e}")
                .red()
                .bold()
                .to_string()
        })? {
            let entry = entry.map_err(|e| {
                format!("Failed to read entry: {e}")
                    .red()
                    .bold()
                    .to_string()
            })?;
            let metadata = entry
                .metadata()
                .map_err(|e| format!("{e}").red().bold().to_string())?;
            total_space_saved += metadata.len().to_f64();

            let path = entry.path();

            if path.is_dir() {
                fs::remove_dir_all(&path).map_err(|e| {
                    format!("Failed to remove directory {path:?}: {e}")
                        .red()
                        .bold()
                        .to_string()
                })?;
            } else {
                fs::remove_file(&path).map_err(|e| {
                    format!("Failed to remove file {path:?}: {e}")
                        .red()
                        .bold()
                        .to_string()
                })?;
            }
        }

        println!("{}", "✓ Trashcan emptied successfully".green());
        println!(
            "You have saved {}",
            human_bytes(total_space_saved).to_string().blue()
        );
        Ok(())
    }
}
