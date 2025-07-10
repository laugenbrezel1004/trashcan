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

    if let Err(e) = trashcan.create_trashcan_directory() {
        return Err(e);
    }
    Ok(trashcan)
}

impl Trashcan {
    fn create_trashcan_directory(&self) -> Result<(), String> {
        if let Err(error) = fs::create_dir(&self.trashcan_homedirectory_location) {
            eprintln!("failed to create trashcan directory: {}", error);
        };
        Ok(())
    }
    pub fn nuke_trashcan_directory(&self) -> Result<(), String> {
        if let Err(e) = fs::remove_dir_all(&self.trashcan_homedirectory_location) {
            eprintln!(
                "You should check your trashcan directory and make sure that it can be created at valid point"
            );
            return Err("trashcan: cannot clean trashcan directory-> {e}".to_string());
        }
        //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
        if let Err(e) = self.create_trashcan_directory() {
            return Err("trashcan: cannot clean trashcan directory-> {e}".to_string());
        };
        Ok(())
    }

    pub fn show_trashcan(&self) -> Result<(), String> {
        let files =
            fs::read_dir(&self.trashcan_homedirectory_location).map_err(|e| e.to_string())?;
        for file in files {
            println!("{}", file.unwrap().file_name().into_string().unwrap());
        }
        Ok(())
        /*
        match files {
            Ok(t) => {
                for file in t {
                    println!("{}", file.unwrap().file_name().into_string().unwrap());
                }
            }
            Err(e) => {
                eprintln!("trashcan: cannot make trashcan directory-> {e}");
            }
        }

         */
    }
}
