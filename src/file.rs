use std::{fs, process};

pub fn check_existence(argument: &str, programname: &str) -> Result<(), String> {
    //if Path::new(argument).exists() {
    match fs::exists(argument) {
        Ok(true) => {
            move_file_to_trashcan(argument)?;
            Ok(())
        }
        //file does not exist
        Ok(false) | Err(_) => {
            return Err(format!(
                "{}: cannot remove '{}': No such file or directory",
                programmname, argument
            ));
        }
    }
}
pub fn move_file_to_trashcan(argument: &str) -> Result<(), String> {
    //TODO: Fehler behandeln
    #[cfg(debug_assertions)]
    println!("destination => {:?}", destination);
    //fs::copy(argument, destination).map_err(|err| err.to_string())?;
    process::Command::new("mv")
        .arg(argument)
        .arg(destination)
        .output()
        .expect("failed to execute mv command");

    Ok(())
}
