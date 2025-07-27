use clap::{Arg, ArgAction, ArgMatches, Command};
use crate::trashcan::Trashcan;
use std::path::Path;

const NUKE_FLAG: &str = "nuke";
const TRASHCAN_FLAG: &str = "trashcan";
const SHOW_TRASHCAN_FLAG: &str = "show-trashcan";
const RESTORE_FLAG: &str = "restore";
const FILES_ARG: &str = "files";

/// Parses command-line arguments using clap.
pub fn parse_args() -> ArgMatches {
    Command::new("trashcan")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("A safer rm replacement that moves files to a trash directory (~/.local/share/trashcan) with UUID tags.")
        .arg(
            Arg::new(NUKE_FLAG)
                .long(NUKE_FLAG)
                .help("Permanently delete files instead of moving to trash")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(TRASHCAN_FLAG)
                .long(TRASHCAN_FLAG)
                .help("Clear the entire trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(SHOW_TRASHCAN_FLAG)
                .long(SHOW_TRASHCAN_FLAG)
                .help("Display contents of the trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(RESTORE_FLAG)
                .long(RESTORE_FLAG)
                .help("Restore the last deleted file from the trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(FILES_ARG)
                .help("Files or directories to delete")
                .num_args(1..)
                .value_name("FILES"),
        )
        .get_matches()
}

/// Processes command-line flags and executes corresponding actions.
pub fn process_flags(trashcan: &Trashcan, matches: ArgMatches) -> Result<(), String> {
    if matches.get_flag(TRASHCAN_FLAG) {
        trashcan.nuke_trashcan_directory()?;
    } else if matches.get_flag(SHOW_TRASHCAN_FLAG) {
        trashcan.show_trashcan()?;
    } else if matches.get_flag(RESTORE_FLAG) {
        trashcan.restore_last_file()?;
    } else if let Some(files) = matches.get_many::<String>(FILES_ARG) {
        let nuke = matches.get_flag(NUKE_FLAG);
        for file in files {
            let path = Path::new(file);
            if !path.exists() {
                return Err(format!("File '{}' does not exist", file));
            }
            if nuke {
                trashcan.nuke_file(file)?;
            } else {
                trashcan.move_file_to_trashcan(file)?;
            }
        }
    } else {
        return Err("No valid action or files specified".to_string());
    }
    Ok(())
}