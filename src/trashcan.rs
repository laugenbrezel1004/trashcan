use std::fs;
use std::process;

// hier noch mehr werte definieren, soll auch Umgebungsvarialben und .config gelesen werden
pub struct Trashcan<'a> {
    pub location: &'a str,
    pub duration: u8,
}
impl Trashcan<'_> {
    pub fn make_trashcan(&self) -> Result<(), ()> {
        #[cfg(debug_assertions)]
        println!("make trashcan");
        if let Err(e) = fs::create_dir(self.location) {
            // wenn verzeichnis schon vorhanden ist soll dieser schritt einfach geskipt werden
            if e.kind() != std::io::ErrorKind::AlreadyExists {
                eprintln!("trashcan: cant create trashcan directory -> {}", e);
                eprintln!(
                    "You should check your trashcan directory and make sure that it can be created at valid point"
                );
                Err(e)
            }
        }

        Ok(())
    }

    pub fn nuke_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("nuke trashcan directory");
        if let Err(e) = fs::remove_dir_all(self.location) {
            eprintln!("trashcan: cannot clean trashcan directory-> {}", e);
            eprintln!(
                "You should check your trashcan directory and make sure that it can be created at valid point"
            );
            process::exit(1);
        }
        //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
        self.make_trashcan();
    }
    pub fn show_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("show trashcan");
        let files = fs::read_dir(self.location);
        match files {
            Ok(t) => {
                for file in t {
                    //println!("{}", file.unwrap().path().display());
                    println!("{}", file.unwrap().file_name().into_string().unwrap());
                }
            }
            Err(e) => {
                eprintln!("trashcan: cannot read trashcan directory -> {}", e);
            }
        }
    }
}
