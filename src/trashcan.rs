// src/trashcan.rs
use chrono::{DateTime, Local};
use humansize::{DECIMAL, format_size};
use owo_colors::OwoColorize;
use std::fs::{self, DirEntry, Metadata};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};
use uuid::Uuid;

/// A safe trashcan implementation with file tracking and restoration capabilities
pub struct Trashcan {
    path: PathBuf,
    info_dir: PathBuf,
}

impl Trashcan {
    /// Initialize the trashcan with proper directory structure
    pub fn initialize() -> Result<Self, String> {
        let home_dir = get_user_by_uid(get_current_uid())
            .ok_or("Failed to get current user")?
            .home_dir()
            .to_path_buf();

        let trash_path = home_dir.join(".local/share/trashcan");
        let info_dir = trash_path.join("info");

        fs::create_dir_all(&info_dir)
            .map_err(|e| format!("Failed to create trashcan info directory: {e}"))?;

        Ok(Self {
            path: trash_path,
            info_dir,
        })
    }

    /// Empty the entire trashcan
    pub fn empty(&self) -> Result<(), String> {
        if !self.path.exists() {
            return Ok(());
        }

        // Remove files
        for entry in
            fs::read_dir(&self.path).map_err(|e| format!("Failed to read trashcan: {e}"))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
            let path = entry.path();

            if path.is_dir() {
                fs::remove_dir_all(&path)
                    .map_err(|e| format!("Failed to remove directory {:?}: {e}", path))?;
            } else {
                fs::remove_file(&path)
                    .map_err(|e| format!("Failed to remove file {:?}: {e}", path))?;
            }
        }

        // Remove info files
        for entry in fs::read_dir(&self.info_dir)
            .map_err(|e| format!("Failed to read info directory: {e}"))?
        {
            let entry = entry.map_err(|e| format!("Failed to read info entry: {e}"))?;
            fs::remove_file(entry.path())
                .map_err(|e| format!("Failed to remove info file: {e}"))?;
        }

        Ok(())
    }

    /// List contents of the trashcan with detailed information
    pub fn list_contents(&self) -> Result<(), String> {
        let mut entries: Vec<(DirEntry, Metadata)> = Vec::new();
        let mut total_size = 0;
        let mut count = 0;

        // Bind colored values to variables first
        let header = "♻️  Trashcan Contents:".bold().bright_blue().to_string();
        let divider = "━".repeat(60).bright_black().to_string();

        println!("{}", header);
        println!("{}", divider);

        for entry in
            fs::read_dir(&self.path).map_err(|e| format!("failed to read trashcan: {}", e))?
        {
            let entry = entry.map_err(|e| format!("failed to read entry: {}", e))?;
            let metadata = entry
                .metadata()
                .map_err(|e| format!("failed to get metadata: {}", e))?;

            total_size += metadata.len();
            count += 1;
            entries.push((entry, metadata));
        }

        // Sort by modification time (newest first)
        entries.sort_by(|a, b| {
            b.1.modified()
                .unwrap_or(SystemTime::UNIX_EPOCH)
                .cmp(&a.1.modified().unwrap_or(SystemTime::UNIX_EPOCH))
        });

        for (i, (entry, metadata)) in entries.iter().enumerate() {
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
                "{:>3}. {:<47} {:>10} {:>8} {}",
                i, bold_name, colored_size, file_type, colored_modified
            );
        }

        //    let index = (i + 1).to_string().bright_yellow();

        //        println!(
        //           "{:>3}. {:<30} {:>10} {:>8} {}",
        //          index, bold_name, colored_size, file_type, colored_modified
        //     );
       println!("{}", divider);

       // Prepare summary line components
   //    let count_str = count.to_string().bold();
    //   let total_size_str = format_size(total_size, DECIMAL).bright_magenta();
     //  println!(
      //     "{} items, total size: {}",
       //    count_str, total_size_str
      // );

   //    if count == 0 {
    //   let empty_msg = "🛑 The trashcan is empty".bold().bright_red();
     //  println ! ("{}", empty_msg);
      // }

       Ok(())

    }

    /// Move file to trashcan with tracking information
    pub fn move_to_trash(&self, file: &str) -> Result<(), String> {
        let src = Path::new(file);
        if !src.exists() {
            return Err(format!("File '{}' does not exist", file.red()));
        }

        let uuid = Uuid::new_v4();
        let dest_name = format!(
            "{}~{}",
            uuid,
            src.file_name().ok_or("Invalid filename")?.to_string_lossy()
        );

        let dest = self.path.join(&dest_name);
        fs::rename(src, &dest).map_err(|e| format!("Failed to move file to trash: {e}"))?;

        // Create trashinfo file (compatible with FreeDesktop.org Trash spec)
        let info_content = format!(
            "[Trash Info]\nPath={}\nDeletionDate={}",
            src.to_string_lossy(),
            chrono::Local::now().format("%Y-%m-%dT%H:%M:%S")
        );

        fs::write(
            self.info_dir.join(format!("{}.trashinfo", uuid)),
            info_content,
        )
        .map_err(|e| format!("Failed to create trash info: {e}"))?;

        Ok(())
    }

    /// Permanently delete a file
    pub fn delete_permanently(&self, file: &str) -> Result<(), String> {
        let path = Path::new(file);
        if path.is_dir() {
            fs::remove_dir_all(path).map_err(|e| format!("Failed to delete directory: {e}"))?;
        } else {
            fs::remove_file(path).map_err(|e| format!("Failed to delete file: {e}"))?;
        }
        Ok(())
    }

    /// Restore the most recently deleted file
    pub fn restore_latest(&self) -> Result<String, String> {
        let mut entries: Vec<DirEntry> = fs::read_dir(&self.path)
            .map_err(|e| format!("Failed to read trashcan: {e}"))?
            .filter_map(Result::ok)
            .collect();

        if entries.is_empty() {
            return Err("Trashcan is empty".to_string());
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
            .last()
            .ok_or("Invalid trashcan file format")?;

        let dest = Path::new(original_name);
        if dest.exists() {
            return Err(format!(
                "Target file '{}' already exists",
                original_name.red()
            ));
        }

        fs::rename(latest.path(), dest).map_err(|e| format!("Failed to restore file: {e}"))?;

        // Clean up the trashinfo file
        if let Some(uuid) = file_name.split('~').next() {
            let _ = fs::remove_file(self.info_dir.join(format!("{}.trashinfo", uuid)));
        }

        Ok(original_name.to_string())
    }
}
