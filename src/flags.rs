use crate::trashcan::Trashcan;
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::Path;

const NUKE_FLAG: &str = "nuke";
const NUKE_TRASHCAN_FLAG: &str = "trashcan";
const SHOW_TRASHCAN_FLAG: &str = "show-trashcan";
const RESTORE_FLAG: &str = "restore";
const FILES_ARG: &str = "files";

/// Parses command-line arguments using clap.
pub fn parse_args() -> clap::ArgMatches {
    Command::new("trashcan")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("A safer rm replacement that moves files to a trash directory (~/.local/share/trashcan) with UUID tags.")
        .arg(
            Arg::new(NUKE_FLAG)
                .long(NUKE_FLAG)
                .help("Permanently delete files instead of moving to trash")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([NUKE_TRASHCAN_FLAG, SHOW_TRASHCAN_FLAG, RESTORE_FLAG]),
        )
        .arg(
            Arg::new(NUKE_TRASHCAN_FLAG)
                .long(NUKE_TRASHCAN_FLAG)
                .help("Clear the entire trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([NUKE_FLAG, SHOW_TRASHCAN_FLAG, RESTORE_FLAG]),
        )
        .arg(
            Arg::new(SHOW_TRASHCAN_FLAG)
                .long(SHOW_TRASHCAN_FLAG)
                .help("Display contents of the trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([NUKE_FLAG, NUKE_TRASHCAN_FLAG, RESTORE_FLAG]),
        )
        .arg(
            Arg::new(RESTORE_FLAG)
                .long(RESTORE_FLAG)
                .help("Restore the last deleted file from the trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all([NUKE_FLAG, NUKE_TRASHCAN_FLAG, SHOW_TRASHCAN_FLAG]),
        )
        .arg(
            Arg::new(FILES_ARG)
                .help("Files to delete")
                .num_args(1..)
                .value_name("FILES")
                .required_unless_present_any([
                    NUKE_TRASHCAN_FLAG,
                    SHOW_TRASHCAN_FLAG,
                    RESTORE_FLAG,
                ]),
        )
        .get_matches()
}

/// Processes command-line flags and executes corresponding actions.
pub fn process_flags(trashcan: &Trashcan, matches: &ArgMatches) -> Result<(), String> {
    if matches.get_flag(NUKE_TRASHCAN_FLAG) {
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
                return Err(format!("Datei '{file}' existiert nicht"));
            }
            if nuke {
                trashcan.nuke_file(file)?;
            } else {
                trashcan.move_file_to_trashcan(file)?;
            }
        }
    } else {
        // Fallback, falls keine Dateien und keine Flags angegeben wurden
        return Err("Keine Dateien oder Aktionen angegeben. Verwende --help für weitere Informationen.".to_string());
    }

    Ok(())
}
