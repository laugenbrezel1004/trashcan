use crate::cli::core::CLI;
use crate::trashcan::core::Trashcan;
use owo_colors::OwoColorize;
use std::path::Path;

/// Handles file operations (moving to trash or permanent deletion)
impl CLI {
    pub fn handle_files(
        &self,
        trashcan: &Trashcan,
        interactive: bool,
        nuke: bool,
        verbose: bool,
    ) -> Result<(), String> {
        let files = self
            .matches
            .get_many::<String>("files")
            .ok_or("No files specified")?;
        let nuke_mode = self.matches.get_flag("nuke");
        let interactive = self.matches.get_flag("interactive");

        for file in files {
            let path = Path::new(file);
            if !path.exists() {
                return Err(format!("File '{}' does not exist", file.red()));
            }

            if interactive {
                let action = if nuke_mode {
                    "permanently delete"
                } else {
                    "move to trash"
                };
                let answer = dialoguer::Confirm::new()
                    .with_prompt(format!("{} {}?", action, file.cyan()))
                    .interact()
                    .map_err(|e| format!("Failed to get user input: {e}"))?;

                if !answer {
                    println!("{} {}", "Skipped:".yellow(), file.cyan());
                    continue;
                }
            }

            if nuke_mode {
                trashcan.delete_permanently(file)?;
                println!("{} {}", "✓ Deleted:".green(), file.cyan());
            } else {
                trashcan.move_to_trash(file, verbose)?;
                println!("{} {}", "✓ Trashed:".green(), file.cyan());
            }
        }

        Ok(())
    }
}
