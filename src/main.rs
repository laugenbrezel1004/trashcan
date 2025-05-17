// src/main.rs

mod file;
mod trashcan;

use clap::{Arg, Command};
use std::env;
use std::path::Path;
use thiserror::Error;
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;
use crate::file::{move_file_to_trashcan, nuke_file};
use crate::trashcan::Trashcan;

// TODO: Check file permission -> fehler ausgeben
// TODO:config file
// TODO:umgebugnsvariablen?
// TODO:mülleimer anzeigen
// TODO: letzte datei wiederherstellen
// TODO: error handeling


fn main()  {
    // Initialize trashcan location
    // get home dir from user
    let mut location = String::new();
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        //location = format!("{}/.local/share/trashcan", user.home_dir().display());
        location = format!("{}/.lasdfocal/share/trashcan", user.home_dir().display());
    }    let trashcan = Trashcan {
        location: &location,
        duration: 10, // TODO: Configurable duration
    };

    // Ensure trashcan directory exists
    trashcan.make_trashcan();

    // Parse command-line arguments
    let matches = Command::new("trashcan")
        .version("rc1.0.0")
        .author("Laurenz Schmidt")
        .about("rm replacement with safe deletion")
        .override_usage("trashcan [FLAGS] <FILES>...")
        .after_help("ENVIRONMENT VARIABLES:\n    None yet")
        .arg(
            Arg::new("nuke")
                .long("nuke")
                .help("destroy file forever and ever - without idiot protection")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .help("Files to delete")
                .num_args(1..)
                .value_parser(clap::value_parser!(String)),
        )
        .arg(
            Arg::new("trashcan")
                .long("trashcan")
                .help("Total Removal of Annoying Stuff, Hella Cleaned, Absolutely Nuked")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Handle nuke deletion
    if matches.get_flag("nuke") {
        if let Some(files) = matches.get_many::<String>("files") {
            for file in files {
                if file::check_existence(file, trashcan.location) {
                    nuke_file(file);
                    log::debug!("Permanently deleted file: {}", file);
                } else {
                    eprintln!("File not found: {}", file);
                }
            }
        }
    } else {
        // Move files to trashcan
        if let Some(files) = matches.get_many::<String>("files") {
            for file in files {
                if file::check_existence(file, trashcan.location) {
                    move_file_to_trashcan(file, trashcan.location);
                    log::debug!("Moved file to trashcan: {}", file);
                } else {
                    eprintln!("File not found: {}", file);
                }
            }
        }
    }

}
