use std::fs;
use std::process;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

// hier noch mehr werte definieren, soll auch Umgebungsvariablen und.config gelesen werden
pub struct Trashcan {
    pub trashcan_homedirectory_location: String,
}

pub fn initialize_trashcan() -> Result<(Trashcan), String> {
    let mut trashcan = Trashcan {
        trashcan_homedirectory_location: get_user_by_uid(get_current_uid())
            .map(|user| format!("{}/.local/share/trashcan", user.home_dir().display()))
            .unwrap_or_else(|| "error".to_string()),
    };
    Ok(trashcan)
}
fn create_trashcan_directory(trashcan: Trashcan) -> Result<(), String> {
    if let Err(error) = fs::create_dir(&trashcan.trashcan_homedirectory_location) {
        eprintln!("failed to create trashcan directory: {}", error);
    };
    Ok(())
}
impl Trashcan {
    pub fn nuke_trashcan_directory(trashcan: Trashcan) -> Result<(), String> {
        if let Err(e) = fs::remove_dir_all(&trashcan.trashcan_homedirectory_location) {
            eprintln!("trashcan: cannot clean trashcan directory-> {e}");
            eprintln!(
                "You should check your trashcan directory and make sure that it can be created at valid point"
            );
            process::exit(1);
        }
        //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
        if let Err(e) = initialize_trashcan() {
            eprintln!("trashcan: cannot clean trashcan directory-> {e}")
        };
    }
    pub fn show_trashcan(trashcan: Trashcan) -> Result<(), String> {
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
}
