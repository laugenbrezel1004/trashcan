use crate::trashcan::core::Trashcan;
use owo_colors::OwoColorize;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::time::SystemTime;

impl Trashcan {
    pub fn restore_latest(&self) -> Result<String, String> {
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.trashcan_path)
            .map_err(|e| {
                format!("Failed to read trashcan: {e}")
                    .red()
                    .bold()
                    .to_string()
            })?
            .filter_map(Result::ok)
            .collect();

        if entries.is_empty() {
            return Err("Trashcan is empty".yellow().to_string());
        }

        // Sort by modification time (newest first)
        entries.sort_by(|a, b| {
            b.metadata()
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .cmp(
                    &a.metadata()
                        .and_then(|m| m.modified())
                        .unwrap_or(SystemTime::UNIX_EPOCH),
                )
        });

        let latest = entries.first().unwrap();
        let file_name = latest.file_name().to_string_lossy().into_owned();

        // Extract original name (remove UUID prefix)
        let original_name = file_name
            .split('~')
            .next_back()
            .ok_or("Invalid trashcan file format".red().bold().to_string())?;

        let dest = Path::new(original_name);
        if dest.exists() {
            return Err(format!(
                "Target file '{}' already exists",
                original_name.red()
            ));
        }

        fs::rename(latest.path(), dest).map_err(|e| {
            format!("Failed to restore file: {e}")
                .red()
                .bold()
                .to_string()
        })?;

        //könnte noch aua machen :/
        println!(
            "{} {}",
            "✓ Restored:".green(),
            latest.path().to_string_lossy().cyan()
        );

        Ok(original_name.to_string())
    }
}
