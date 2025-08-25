use crate::trashcan::core::Trashcan;
use chrono::{DateTime, Local};
use humansize::{DECIMAL, format_size};
use owo_colors::OwoColorize;
use std::fs;
use std::fs::{DirEntry, Metadata};
use std::time::SystemTime;

impl Trashcan {
    pub fn list_contents(&self) -> Result<(), String> {
        let mut is_empty: bool = true;
        let mut entries: Vec<(DirEntry, Metadata)> = Vec::new();
        //TODO: show also the size of the trashcan directory
        let mut total_size = 0;
        let mut count = 0;

        // Bind colored values to variables first
        //TODO: Icon for the trashcan? Still problems with rendering the trashcan from nerfonts
        let header = "Trashcan Contents:".bold().bright_blue().to_string();
        let divider = "━".repeat(60).bright_black().to_string();

        println!("{header}");
        println!("{divider}");
        for entry in fs::read_dir(&self.trashcan_path)
            .map_err(|e| format!("failed to read trashcan: {e}"))?
        {
            let entry = entry.map_err(|e| format!("failed to read entry: {e}"))?;
            let metadata = entry
                .metadata()
                .map_err(|e| format!("failed to get metadata: {e}"))?;
            total_size += metadata.len();
            count += 1;
            entries.push((entry, metadata));
        }

        // Sort by modification time (newest first)
        // I don't know how this works, it just works, so don't change it please :)
        entries.sort_by(|a, b| {
            b.1.modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .cmp(&a.1.modified().unwrap_or(SystemTime::UNIX_EPOCH))
        });

        for (i, (entry, metadata)) in entries.iter().enumerate() {
            is_empty = false;
            let name = entry.file_name().to_string_lossy().to_string();
            let bold_name = name.bold().to_string(); // Convert to String
            let size = format_size(metadata.len(), DECIMAL);
            let colored_size = size.bright_magenta();
            let modified = metadata
                .modified()
                .map(|t| {
                    DateTime::<Local>::from(t)
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                })
                .unwrap_or_else(|_| "unknown".to_string());
            let colored_modified = modified.bright_black();

            // First create the base string, then apply color
            let file_type = if metadata.is_dir() {
                "📁 dir".to_string()
            } else if metadata.is_file() {
                "📄 file".to_string()
            } else if metadata.file_type().is_symlink() {
                "🔗 symlink".to_string()
            } else {
                "� unknown".to_string()
            };


                println!(
                   "{:>3}. {:<30} {:>10} {:>8} {}",
                  i.yellow(), bold_name, colored_size, file_type, colored_modified
             );
        }
        if is_empty {
            println!("Your trashcan seems clean as fuck!");
        }

        // Prepare summary line components
        let count_str = count.to_string();
        let total_size_str = format_size(total_size, DECIMAL);
        println!(
            "{} items, total size: {}",
            count_str.yellow(), total_size_str.bright_magenta()
        );

        println!("{divider}");

        if count == 0 {
            let empty_msg = "🛑 The trashcan is empty".bold();
            println ! ("{}", empty_msg.bright_red());
        }

        Ok(())
    }
}
