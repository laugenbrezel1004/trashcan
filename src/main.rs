// src/main.rs

mod file;
mod flags;
mod trashcan;

use crate::file::{move_file_to_trashcan, nuke_file};
use clap::{Arg, ArgAction, Command};
use users::os::unix::{UserExt};
use users::{get_current_uid, get_user_by_uid, User};
/*
TODO: Check file permission -> fehler ausgeben | wichtig wenn eigene implementation ohne "mv"
TODO:config file
TODO:umgebugnsvariablen?
TODO:mülleimer anzeigen -> typ anzeigen? größe vom eimer anzeigen?
TODO: letzte datei wiederherstellen
TODO: autocompletion in cmd
TODO: mv mit "guten" code ersetzen
*/

#[cfg(target_os = "linux")]
fn main() {
    //get a trashcan
    // get trashcandir based on the USER
    // create trashcandir based on the USER
    // Trashcan initialisieren
    // move on with checking the flags via clap lib
    if let Err(e) = flags::core::check_flags(trashcan::core::initialize_trashcan()){
        eprintln!("trashcan: cannot check flags {e}");
        std::process::exit(1);
    }
}
