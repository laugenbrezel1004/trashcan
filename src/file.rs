use chrono::Local;
use std::{fs, process};
pub fn check_existence(argument: &str, programname: &str, location: &str) {
    match fs::exists(argument) {
        Ok(true) => {
            move_file_to_trashcan(argument, location);
        }
        //file does not exist
        Ok(false) | Err(_) => {
            eprintln!(
                "{}: cannot remove '{}': No such file or directory",
                programname, argument
            );
        }
    }
}
pub fn move_file_to_trashcan(argument: &str, location: &str) {
    //TODO: Fehler behandeln
    #[cfg(debug_assertions)]
    println!("destination => {:?}", location);
    let suffix_time = Local::now().format("%H:%M:%S").to_string();
    let location = format!("{}/{}{}", location, argument, suffix_time);
    //fs::copy(argument, destination).map_err(|err| err.to_string())?;
    process::Command::new("mv")
        .arg(argument)
        .arg(location)
        .output()
        .expect("failed to execute mv command");
}