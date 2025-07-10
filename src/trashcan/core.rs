
use std::fs;
use std::process;
use users::{get_current_uid, get_user_by_uid};

// hier noch mehr werte definieren, soll auch Umgebungsvariablen und.config gelesen werden
pub struct Trashcan {
}


pub fn initialize_trashcan() -> Result<(), String> {
    // get home dir from user
    let mut home_directory = String::new();
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        // Initialize trashcan location
        home_directory = format!("{}/.local/share/trashcan", user.home_dir().display());
    }
    #[cfg(debug_assertions)]
    println!("Home directory: {}", &home_directory);

    let trashcan = Trashcan {
    };
    if let Err(e) = trashcan.make_trashcan() {
        return Err(e.to_string());
    }
    Ok(trashcan)
}
impl Trashcan {
    pub fn make_trashcan_dir(&self) -> Result<(), String> {
        if let Err(e) = fs::create_dir(location) {
            // wenn verzeichnis schon vorhanden ist, soll dieser schritt einfach geskippt werden
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                let error = format!("{} {}", "trashcan: cant create trashcan directory -> {}\
                \nYou should check your trashcan directory and make sure that it can be created \
                at valid point", &self.location);
                return Err(error);
            }
            return Ok(()); //directory already exists
        }
    }
    pub fn nuke_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("nuke trashcan directory");
        if let Err(e) = fs::remove_dir_all(&self.location) {
            eprintln!("trashcan: cannot clean trashcan directory-> {e}");
            eprintln!(
                "You should check your trashcan directory and make sure that it can be created at valid point"
            );
            process::exit(1);
        }
        //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
        if let Err(e)  = self.make_trashcan() {
            eprintln!("trashcan: cannot clean trashcan directory-> {e}");
        }
    }
    pub fn show_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("show trashcan");
        let files = fs::read_dir(&self.location);
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
