use std::{fs, process};
use uuid::Uuid;

pub fn check_existence(argument: &str, location: &str) -> bool {
    return match fs::exists(argument) {
        Ok(true) => true,
        //file does not exist
        Ok(false) => {
            eprintln!(
                "trashcan: cannot remove '{}': No such file or directory",
                argument
            );
            false
        }
        Err(e) => {
            eprintln!("trashcan: cannot remove '{}': {}", argument, e);
            false
        }
    };
}
pub fn move_file_to_trashcan(argument: &str, location: &str) {
    #[cfg(debug_assertions)]
    println!("destination => {:?}", location);
    let suffix_uuid = Uuid::new_v4();
    let location = format!("{}/{}:{}", location, argument, suffix_uuid); // location e.g. /home/laurenz/.local/share/trashcan/deletefile10:10:10
    //fs::copy(argument, destination).map_err(|err| err.to_string())?;
    process::Command::new("mv")
        .arg(argument)
        .arg(location)
        .output()
        .expect("failed to execute mv command");
}
pub fn nuke_file(argument: &str) {
    if let Err(e) = std::fs::remove_file(argument) {
        eprintln!("trashcan: cannot remove '{}': {}", argument, e);
    }
}
