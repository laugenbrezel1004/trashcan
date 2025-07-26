use std::{fs};


#[allow(unused)]
pub fn check_existence(argument: &String) -> bool {
    match fs::metadata(argument) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("trashcan: cannot remove '{argument}': {e}");
            false
        }
    }
}


#[allow(unused)]
pub fn nuke_file(argument: &String) -> Result<(), String> {
    if let Err(e) = std::fs::remove_file(argument) {
        eprintln!("trashcan: cannot remove '{argument}': {e}");
    }
    Ok(())
}
