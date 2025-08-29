use super::super::core::Trashcan;
use crate::utils;
use owo_colors::OwoColorize;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl Trashcan {
    pub fn move_to_trash(&self, infile: &str, verbose: bool) -> Result<(), String> {
        let src = Path::new(infile);
        if !src.exists() {
            return Err(format!("File '{}' does not exist", infile.red().bold()));
        }

        // Generate shorter timestamp (seconds since epoch in hex)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs(); // Use seconds instead of milliseconds

        let original_filename = src.file_name().ok_or("Invalid filename")?.to_string_lossy();

        // Extract file stem and extension for cleaner naming
        let stem = src
            .file_stem()
            .unwrap_or_else(|| std::ffi::OsStr::new("file"))
            .to_string_lossy();

        let extension = src
            .extension()
            .map(|ext| format!(".{}", ext.to_string_lossy()))
            .unwrap_or_default();

        // Format: stem_timestamp.extension (shorter hex format)
        let outfile = format!("{}_{:x}{}", stem, timestamp, extension); // {:x} = hexadecimal

        //TODO:
        let dest = self.trashcan_path.join(&outfile);

        // Handle cross-filesystem moves with fallback
        if let Err(rename_err) = fs::rename(src, &dest) {
            // Fallback: copy + delete for cross-filesystem moves
            fs::copy(src, &dest).map_err(|copy_err| {
                format!(
                    "Failed to move file (rename: {}, copy: {})",
                    rename_err.to_string().red().bold(),
                    copy_err.to_string().red().bold()
                )
            })?;

            // Only remove original if copy succeeded
            fs::remove_file(src).map_err(|remove_err| {
                format!(
                    "Copied to trash but failed to remove original: {}",
                    remove_err.to_string().red().bold()
                )
            })?;

            utils::vprint(
                format!(
                    "Copied '{}' to trash (cross-filesystem) as '{}'",
                    original_filename.green(),
                    outfile.blue(),
                ),
                verbose,
            );
        } else {
            utils::vprint(
                format!(
                    "Moved '{}' to trash as '{}'",
                    original_filename.green(),
                    outfile.blue()
                ),
                verbose,
            );
        }
        utils::vprint(
            format!("{} {}", "✓ Trashed:".green(), outfile.cyan()),
            verbose,
        );

        Ok(())
    }
}
