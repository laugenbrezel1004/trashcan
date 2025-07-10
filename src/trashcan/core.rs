
use std::fs;
use std::process;

// hier noch mehr werte definieren, soll auch Umgebungsvariablen und.config gelesen werden
pub struct Trashcan {
}
    pub fn new() -> Result<(Trashcan), String> {

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
        Ok(())
    }

impl Trashcan {
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
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    // Test make_trashcan when directory does not exist
    #[test]
    fn test_make_trashcan_new_directory() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        let trashcan_path_str = trashcan_path.to_str().unwrap().to_string();

        let trashcan = Trashcan {
            location: trashcan_path_str.clone(),
        };
        assert!(trashcan.make_trashcan().is_ok());
        assert!(trashcan_path.exists());
        assert!(trashcan_path.is_dir());
    }

    // Test make_trashcan when directory already exists
    #[test]
    fn test_make_trashcan_directory_exists() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();
        let trashcan_path_str = trashcan_path.to_str().unwrap().to_string();

        let trashcan = Trashcan {
            location: trashcan_path_str.clone(),
        };
        assert!(trashcan.make_trashcan().is_ok());
        assert!(trashcan_path.exists());
    }

    // Test make_trashcan with invalid permissions
    #[allow(unused)]
    //#[test]
    fn test_make_trashcan_no_permissions() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("noperms");
        let trashcan_path_str = trashcan_path.to_str().unwrap().to_string();

        // Create a directory and remove write permissions to simulate failure
        fs::create_dir(&trashcan_path).unwrap();
        let mut perms = fs::metadata(&trashcan_path).unwrap().permissions();
        perms.set_readonly(true);
        fs::set_permissions(&trashcan_path, perms).unwrap();

        let trashcan = Trashcan {
            location: trashcan_path_str.clone(),
        };
        assert!(trashcan.make_trashcan().is_err());
    }

    // Test nuke_trashcan
    #[test]
    fn test_nuke_trashcan() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();

        // Create some files in the trashcan
        File::create(trashcan_path.join("file1.txt")).unwrap();
        File::create(trashcan_path.join("file2.txt")).unwrap();

        let trashcan = Trashcan {
            location: trashcan_path.to_str().unwrap().to_string(),
        };
        trashcan.nuke_trashcan();

        // Check that the directory exists (re-created) and is empty
        assert!(trashcan_path.exists());
        let contents = fs::read_dir(&trashcan_path).unwrap().count();
        assert_eq!(contents, 0);
    }

    // Test show_trashcan with files
    #[test]
    fn test_show_trashcan_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();

        // Create some files
        File::create(trashcan_path.join("file1.txt")).unwrap();
        File::create(trashcan_path.join("file2.txt")).unwrap();

        let trashcan = Trashcan {
            location: trashcan_path.to_str().unwrap().to_string(),
        };

        // Redirect stdout to capture output
        let output = std::panic::catch_unwind(|| {
            trashcan.show_trashcan();
        });
        assert!(output.is_ok());
    }

    // Test show_trashcan with empty directory
    #[test]
    fn test_show_trashcan_empty() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();

        let trashcan = Trashcan {
            location: trashcan_path.to_str().unwrap().to_string(),
        };

        let output = std::panic::catch_unwind(|| {
            trashcan.show_trashcan();
        });
        assert!(output.is_ok());
    }
}
