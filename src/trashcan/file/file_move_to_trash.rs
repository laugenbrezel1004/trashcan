use super::super::core::Trashcan;
use crate::utils;
use owo_colors::OwoColorize;
use std::fs;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl Trashcan {
    pub fn move_to_trash(&self, infile: &str, verbose: bool) -> Result<(), String> {
        let src = Path::new(infile);

        // Check if file exists
        if !src.exists() {
            return Err(format!("File '{}' does not exist", infile.red().bold()));
        }

        // Check if we have read permission for the source file
        if let Err(e) = fs::metadata(src) {
            return Err(format!(
                "Cannot access '{}': {}",
                infile.red().bold(),
                e.to_string().red().bold()
            ));
        }

        // Generate shorter timestamp (seconds since epoch in hex)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| format!("Time error: {}", e))?
            .as_secs();

        let original_filename = src.file_name().ok_or("Invalid filename")?.to_string_lossy();

        // Format: stem_timestamp.extension (shorter hex format)
        let outfile = format!("{}_{:x}", original_filename, timestamp); // {:x} = hexadecimal
        let destination = self.trashcan_path.join(&outfile);

        // Check if we have write permission in trash directory
        if let Err(e) = fs::metadata(&self.trashcan_path) {
            return Err(format!(
                "Cannot access trash directory '{}': {}",
                self.trashcan_path.display().to_string().red().bold(),
                e.to_string().red().bold()
            ));
        }

        // Handle cross-filesystem moves with fallback
        if let Err(rename_err) = fs::rename(src, &destination) {
            // Check if it's a permission error specifically
            if rename_err.kind() == io::ErrorKind::PermissionDenied {
                return Err(format!(
                    "Permission denied: Cannot move '{}' to trash",
                    infile.red().bold(),
                ));
            }

            // Fallback: copy + delete for cross-filesystem moves
            fs::copy(src, &destination).map_err(|copy_err| {
                // Handle permission errors during copy
                if copy_err.kind() == io::ErrorKind::PermissionDenied {
                    format!(
                        "Permission denied: Cannot copy '{}' to trash directory '{}'",
                        infile.red().bold(),
                        self.trashcan_path.display().to_string().red().bold()
                    )
                } else {
                    format!(
                        "Failed to copy file '{}' to trash: {}",
                        infile.red().bold(),
                        copy_err.to_string().red().bold()
                    )
                }
            })?;

            // Only remove original if copy succeeded
            fs::remove_file(src).map_err(|remove_err| {
                // Handle permission errors during removal
                if remove_err.kind() == io::ErrorKind::PermissionDenied {
                    format!(
                        "Copied to trash but permission denied to remove original file '{}'",
                        infile.red().bold()
                    )
                } else {
                    format!(
                        "Copied to trash but failed to remove original file '{}': {}",
                        infile.red().bold(),
                        remove_err.to_string().red().bold()
                    )
                }
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
