// src/main.rs

mod file;
mod trashcan;

use crate::file::{move_file_to_trashcan, nuke_file};
use crate::trashcan::Trashcan;
use clap::{Arg, Command, ArgAction};
use std::path::PathBuf;
use thiserror::Error;
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;



fn main() {
    // Trashcan initialisieren
    let trashcan = initialize_trashcan();

    // Kommandozeilenargumente parsen
    let matches = Command::new("trashcan")
        .version("1.0.2")
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
        .arg(
            Arg::new("files")
                .help("Files to delete")
                .num_args(1..),
        )
        .get_matches();

    // Aktionen ausführen
    if matches.get_flag("trashcan") {
        trashcan.nuke_trashcan();
    } else if matches.get_flag("show-trashcan") {
        trashcan.show_trashcan();
    } else if let Some(files) = matches.get_many::<String>("files") {
        let action = if matches.get_flag("nuke") {
            nuke_file
        } else {
            |file: &str| move_file_to_trashcan(file, &trashcan.location)
        };
        for file in files {
            if file::check_existence(file, &trashcan.location) {
                action(file);
                log::debug!("Processed file: {}", file);
            } else {
                eprintln!("File not found: {}", file);
            }
        }
    }
}

fn initialize_trashcan() -> Result<Trashcan, ()> {
    // Home-Verzeichnis ermitteln
    let home_dir = get_user_by_uid(get_current_uid())
        .home_dir()
        .to_str()
        .ok_or(TrashcanError::InvalidHomeDir)?;

    let location = format!("{}/.local/share/trashcan", home_dir);

    // Trashcan erstellen
    let trashcan = Trashcan {
        location: &location,
        duration: 10, // TODO: Konfigurierbar machen
    };
    trashcan.make_trashcan();
    Ok(trashcan)
}