// src/cli.rs
use crate::trashcan::core::Trashcan;
use clap::{Arg, ArgAction, ArgMatches, Command};
use owo_colors::OwoColorize;
use std::path::Path;

/// Command Line Interface handler for the trashcan utility
pub struct Cli {
    pub matches: ArgMatches,
}

impl Cli {
    /// Creates a new CLI instance with all command line arguments configured
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
                        .long("empty_trash")
                        .short('e')
                        .help("Empty the entire trashcan")
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["nuke", "show_trashcan", "restore"]),
                )
                .arg(
                    Arg::new("show_trashcan")
                        .long("show_trashcan")
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
    /// Executes the appropriate action based on command line arguments
    pub fn run(&self) -> Result<(), String> {
        let trashcan = Trashcan::initialize()?;

        // check the  ok thingy later on i dont know if it can breakt somehing
        match (
            self.matches.get_flag("interactive"),
            self.matches.get_flag("nuke"),
            // stand alone flags
            self.matches.get_flag("empty_trash"),
            self.matches.get_flag("show_trashcan"),
            self.matches.get_flag("restore"),
        ) {
            (true, true, false, false, false) => trashcan.remove_garbage(true)?,
            (_, true, false, false, _) => trashcan.remove_garbage(false)?, // TODO: Sicherheitsmechanismus hinzufügen
            (_, false, true, false) => trashcan.list_contents()?,
            (_, false, false, true) => trashcan.restore()?,
            // i'll do that shit when i get it what is does exactly
            // ich bin gebrochen :/
            _ => self.handle_files(&trashcan)?,
            // if there is no flag, delete the given files - if the user sumbmits no files, nor flags, clap should get the error
            // and prombt the user a error message
            // _ => println!(".?>"),
        }
        Ok(())
    }
}
