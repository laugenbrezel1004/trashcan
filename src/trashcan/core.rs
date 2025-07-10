use std::fs;
use std::process;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

// hier noch mehr werte definieren, soll auch Umgebungsvariablen und.config gelesen werden
pub  TRASHCAN_DIRECTORY: String = get_user_by_uid(get_current_uid())
    .map(|user| format!("{}/.local/share/trashcan", user.home_dir().display()))
    .unwrap_or_else(|| "error".to_string());

pub fn initialize_trashcan() -> Result<(), String> {
    if let Err(error) = fs::create_dir(&TRASHCAN_DIRECTORY) {
        return Err(format!("failed to create trashcan directory: {}", error));
    }
    Ok(())
}
pub fn nuke_trashcan() {
    if let Err(e) = fs::remove_dir_all(&TRASHCAN_DIRECTORY) {
        eprintln!("trashcan: cannot clean trashcan directory-> {e}");
        eprintln!(
            "You should check your trashcan directory and make sure that it can be created at valid point"
        );
        process::exit(1);
    }
    //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
    if let Err(e) = initialize_trashcan() {
        eprintln!("trashcan: cannot clean trashcan directory-> {e}");
    }
}
pub fn show_trashcan() {
    let files = fs::read_dir(&TRASHCAN_DIRECTORY);
    match files {
        Ok(t) => {
            for file in t {
                //println!("{}", file.unwrap().path().display());
                println!("{}", file.unwrap().file_name().into_string().unwrap());
            }
        }
        Err(e) => {
            eprintln!("trashcan: cannot make trashcan directory-> {e}");
        }
    }
}
