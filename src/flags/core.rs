use clap::{Arg, ArgAction, Command};
use crate::{file, trashcan};
use crate::file::{move_file_to_trashcan, nuke_file};

pub fn check_flags() {
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
        trashcan::core::nuke_trashcan();
    } else if matches.get_flag("show-trashcan") {
        //TODO: Feherl behandeln
        trashcan::core::show_trashcan();
    } else if let Some(files) = matches.get_many::<String>("files") {
        for file in files {
            if file::check_existence(file) {
                if matches.get_flag("nuke") {
                    nuke_file(file);
                } else if matches.get_flag("files") {
                    move_file_to_trashcan(file, &trashcan::core::TRASHCAN_DIRECTORY);
                }
            } else {
                eprintln!("File not found: {file}");
            }
        }
    }
}
