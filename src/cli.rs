// src/cli.rs
use crate::trashcan::Trashcan;
use clap::{Arg, ArgAction, ArgMatches, Command};
use owo_colors::OwoColorize;
use std::path::Path;
use dialoguer;

pub struct Cli {
    pub matches: ArgMatches,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            matches: Command::new("trashcan")
                .version(env!("CARGO_PKG_VERSION"))
                .author("Laurenz Schmidt")
                .about("A safer rm replacement with trashcan functionality")
                .long_about("A modern alternative to rm that moves files to a trash directory (~/.local/share/trashcan) instead of permanent deletion.")
                .after_help("Examples:\n  trashcan file.txt            # Move file to trash\n  trashcan --show-trashcan     # List trash contents\n  trashcan --restore           # Restore last deleted file\n  trashcan --nuke file.txt     # Permanently delete file")
                .arg(
                    Arg::new("nuke")
                        .long("nuke")
                        .short('n')
                        .help("Permanently delete files instead of moving to trash")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["empty_trash", "show_trashcan", "restore"]),
                )
                .arg(
                    Arg::new("empty_trash")
                        .long("empty-trash")
                        .short('e')
                        .help("Empty the entire trashcan")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["nuke", "show_trashcan", "restore"]),
                )
                .arg(
                    Arg::new("show_trashcan")
                        .long("show-trashcan")
                        .short('l')
                        .help("Show contents of the trashcan")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["nuke", "empty_trash", "restore"]),
                )
                .arg(
                    Arg::new("restore")
                        .long("restore")
                        .short('r')
                        .help("Restore the last deleted file from trashcan")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["nuke", "empty_trash", "show_trashcan"]),
                )
                .arg(
                    Arg::new("interactive")
                        .long("interactive")
                        .short('i')
                        .help("Prompt before each deletion")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("files")
                        .help("Files or directories to operate on")
                        .num_args(1..)
                        .value_name("FILES")
                        .required_unless_present_any(["empty_trash", "show_trashcan", "restore"]),
                )
                .get_matches(),
        }
    }

    pub fn run(&self) -> Result<(), String> {
        let trashcan = Trashcan::initialize()?;

        if self.matches.get_flag("empty_trash") {
            self.empty_trash(&trashcan)
        } else if self.matches.get_flag("show_trashcan") {
            self.show_trash(&trashcan)
        } else if self.matches.get_flag("restore") {
            self.restore_file(&trashcan)
        } else {
            self.handle_files(&trashcan)
        }
    }

    fn empty_trash(&self, trashcan: &Trashcan) -> Result<(), String> {
        if self.matches.get_flag("interactive") {
            let answer = dialoguer::Confirm::new()
                .with_prompt("Are you sure you want to empty the trashcan?")
                .interact()
                .map_err(|e| format!("Failed to get user input: {e}"))?;

            if !answer {
                println!("{}", "Operation cancelled".yellow());
                return Ok(());
            }
        }

        trashcan.empty()?;
        println!("{}", "✓ Trashcan emptied successfully".green());
        Ok(())
    }

    fn show_trash(&self, trashcan: &Trashcan) -> Result<(), String> {
        trashcan.list_contents()
    }

    fn restore_file(&self, trashcan: &Trashcan) -> Result<(), String> {
        let restored = trashcan.restore_latest()?;
        println!("{} {}", "✓ Restored:".green(), restored.cyan());
        Ok(())
    }

    fn handle_files(&self, trashcan: &Trashcan) -> Result<(), String> {
        let files = self.matches.get_many::<String>("files")
            .ok_or("No files specified")?;
        let nuke_mode = self.matches.get_flag("nuke");
        let interactive = self.matches.get_flag("interactive");

        for file in files {
            let path = Path::new(file);
            if !path.exists() {
                return Err(format!("File '{}' does not exist", file.red()));
            }

            if interactive {
                let action = if nuke_mode { "permanently delete" } else { "move to trash" };
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
                trashcan.move_to_trash(file)?;
                println!("{} {}", "✓ Trashed:".green(), file.cyan());
            }
        }

        Ok(())
    }
}