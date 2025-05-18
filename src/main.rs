// src/main.rs

mod file;
mod trashcan;

use crate::file::{move_file_to_trashcan, nuke_file};
use crate::trashcan::Trashcan;
use clap::{Arg, ArgAction, Command};
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};



// TODO: Check file permission -> fehler ausgeben
// TODO:config file
// TODO:umgebugnsvariablen?
// TODO:mülleimer anzeigen -> typ anzeigen? größe vom eimer anzeigen?
// TODO: letzte datei wiederherstellen
// TODO: autocompletion in cmd

#[cfg(target_os = "linux")]
fn main() {
    // Trashcan initialisieren
    match initialize_trashcan() {
        Ok(trash) => {
            check_flags(trash);
        }
        // sollte nicht eintreffen, sollt
        Err(e) => {
            eprintln!("trashcan: Something went horribly wrong: {}", e);
            std::process::exit(1);
        }
    }
}

fn initialize_trashcan() -> Result<Trashcan, String> {
    // get home dir from user
    let mut home_directory = String::new();
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        // Initialize trashcan location
        home_directory = format!("{}/.local/share/trashcan", user.home_dir().display());
    }
    #[cfg(debug_assertions)]
    println!("Home directory: {}", &home_directory);

    // Trashcan erstellen
    let trashcan = Trashcan {
        location: home_directory,
        // duration: 10, // TODO: Konfigurierbar machen
    };
    if let Err(e) = trashcan.make_trashcan() {
        return Err(e.to_string());
    }
    Ok(trashcan)
}
fn check_flags(trashcan: Trashcan) {
    // Kommandozeilenargumente parsen
    let matches = Command::new("trashcan")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("rm replacement with safe deletion")
        .arg(
            Arg::new("nuke")
                .long("nuke")
                .help("Permanently delete files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("trashcan")
                .long("trashcan")
                .help("Clear the entire trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-trashcan")
                .long("show-trashcan")
                .help("Show trashcan contents")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new("files").help("Files to delete").num_args(1..))
        .get_matches();

    // Aktionen ausführen
    if matches.get_flag("trashcan") {
        //TODO: Feherl behandeln
        trashcan.nuke_trashcan();
    } else if matches.get_flag("show-trashcan") {
        //TODO: Feherl behandeln
        trashcan.show_trashcan();
    } else if let Some(files) = matches.get_many::<String>("files") {
        for file in files {
            if file::check_existence(file) {
                if matches.get_flag("nuke") {
                    nuke_file(file);
                } else if matches.get_flag("files") {
                    move_file_to_trashcan(file, &trashcan.location);
                }
            } else {
                eprintln!("File not found: {}", file);
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Test initialize_trashcan
    //#[test]
    #[allow(unused)]
    fn test_initialize_trashcan() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_str().unwrap().to_string();
        // Mock user home directory by temporarily overriding environment variable
        unsafe {
            std::env::set_var("HOME", &home_dir);
        }

        let result = initialize_trashcan();
        assert!(result.is_ok());
        #[allow(unused)]
        let trashcan = result.unwrap();
        let expected_path = format!("{}/.local/share/trashcan", home_dir);
        //assert_eq!(trashcan.location, expected_path);

        // Check that the directory was created
        assert!(fs::metadata(&expected_path).is_ok());
    }

    // Test check_flags with --nuke
    #[allow(unused)]
    //#[test]
    fn test_check_flags_nuke() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();
        let trashcan = Trashcan {
            location: trashcan_path.to_str().unwrap().to_string(),
        };

        // Create a test file
        let file_path = temp_dir.path().join("testfile.txt");
        fs::write(&file_path, "test content").unwrap();
        let file_path_str = file_path.to_str().unwrap().to_string();

        // Simulate clap matches with --nuke and a file
        Command::new("trashcan")
            .arg(Arg::new("nuke").long("nuke").action(ArgAction::SetTrue))
            .arg(Arg::new("files").num_args(1..))
            .get_matches_from(vec!["trashcan", "--nuke", &file_path_str]);

        check_flags(trashcan);

        // File should be deleted
        assert!(!file_path.exists());
    }

    // Test check_flags with --trashcan
    #[allow(unused)]
    //#[test]
    fn test_check_flags_clear_trashcan() {
        let temp_dir = TempDir::new().unwrap();
        let trashcan_path = temp_dir.path().join("trashcan");
        fs::create_dir(&trashcan_path).unwrap();

        // Create a file in the trashcan
        fs::write(trashcan_path.join("file.txt"), "test").unwrap();

        let trashcan = Trashcan {
            location: trashcan_path.to_str().unwrap().to_string(),
        };

        // Simulate clap matches with --trashcan
        Command::new("trashcan")
            .arg(
                Arg::new("trashcan")
                    .long("trashcan")
                    .action(ArgAction::SetTrue),
            )
            .get_matches_from(vec!["trashcan", "--trashcan"]);

        check_flags(trashcan);

        // Trashcan should exist and be empty
        assert!(trashcan_path.exists());
        let contents = fs::read_dir(&trashcan_path).unwrap().count();
        assert_eq!(contents, 0);
    }
}
