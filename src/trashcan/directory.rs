use crate::trashcan::core::Trashcan;
use std::fs;

impl Trashcan {
    pub fn create_trashcan_directory(&self) -> Result<(), String> {
        if let Err(error) = fs::create_dir(&self.trashcan_homedirectory_location) {
            eprintln!("trashcan: failed to create trashcan directory: {error}");
        };
        Ok(())
    }
    pub fn nuke_trashcan_directory(&self) -> Result<(), String> {
        if let Err(e) = fs::remove_dir_all(&self.trashcan_homedirectory_location) {
            eprintln!(
                "You should check your trashcan directory and make sure that it can be created at valid point"
            );
            return Err(format!("trashcan: cannot clean trashcan directory: {e}"));
        }
        //remove_dir_all löscht auch das verzeichnis, hier wird das trashcan verzeichnis wieder angelegt, nachdem "der inhalt" gelöscht wurde
        self.create_trashcan_directory()?;
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
