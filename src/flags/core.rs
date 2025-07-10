use crate::file::{move_file_to_trashcan, nuke_file};
use crate::trashcan::core::Trashcan;
use crate::{file, trashcan};
use clap::{Arg, ArgAction, Command};

pub fn check_flags(trashcan: Trashcan) -> Result<(), String> {
    // Kommandozeilenargumente parsen
    let matches = Command::new("trashcan")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("rm replacement with safe deletion")
        .arg(
            Arg::new("nuke")
                .long("nuke")
                .help("Permanently delete files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("trashcan")
                .long("trashcan")
                .help("Clear the entire trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-trashcan")
                .long("show-trashcan")
                .help("Show trashcan contents")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("files").help("Files to delete").num_args(1..))
        .get_matches();

    // Aktionen ausführen
    if matches.get_flag("trashcan") {
        //TODO: Feherl behandeln
        trashcan.nuke_trashcan_directory()?
    } else if matches.get_flag("show-trashcan") {
        //TODO: Feherl behandeln
        trashcan.show_trashcan()?
    }
    Ok(())
}
