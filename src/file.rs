use std::{fs, process};
use uuid::Uuid;

pub fn check_existence(argument: &String) -> bool {
    match fs::metadata(argument) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("trashcan: cannot remove '{}': {}", argument, e);
            false
        }
    }
}

pub fn move_file_to_trashcan(argument: &String, location: &str) {
    #[cfg(debug_assertions)]
    println!("destination => {:?}", location);
    let suffix_uuid = Uuid::new_v4();
    let location = format!("{}/{}:{}", location, argument, suffix_uuid); // location e.g. /home/laurenz/.local/share/trashcan/deletefile10:10:10
    //fs::copy(argument, destination).map_err(|err| err.to_string())?;
    process::Command::new("mv")
        .arg(argument)
        .arg(location)
        .output()
        .expect("failed to execute mv command");
}
pub fn nuke_file(argument: &String) {
    if let Err(e) = std::fs::remove_file(argument) {
        eprintln!("trashcan: cannot remove '{}': {}", argument, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    // Test check_existence for an existing file
    #[test]
    fn test_check_existence_file_exists() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("testfile.txt");
        File::create(&file_path).unwrap();

        let file_path_str = file_path.to_str().unwrap().to_string();
        assert!(check_existence(&file_path_str));
    }

    // Test check_existence for a non-existing file
    #[test]
    fn test_check_existence_file_does_not_exist() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");
        let file_path_str = file_path.to_str().unwrap().to_string();
        assert!(!check_existence(&file_path_str));
    }

    // Test move_file_to_trashcan
    #[allow(unused)]
    //#[test]
    fn test_move_file_to_trashcan() {
        let temp_dir = TempDir::new().unwrap();
        let trash_dir = temp_dir.path().join("trashcan");
        fs::create_dir(&trash_dir).unwrap();

        // Create a test file
        let source_file = temp_dir.path().join("source.txt");
        File::create(&source_file)
            .unwrap()
            .write_all(b"test content")
            .unwrap();
        let source_file_str = source_file.to_str().unwrap().to_string();
        let trash_dir_str = trash_dir.to_str().unwrap().to_string();

        // Move the file to the trashcan
        move_file_to_trashcan(&source_file_str, &trash_dir_str);

        // Check that the source file no longer exists
        assert!(!source_file.exists());

        // Check that a file exists in the trashcan with the expected prefix
        let trash_contents = fs::read_dir(&trash_dir).unwrap();
        let moved_file = trash_contents
            .filter_map(Result::ok)
            .find(|entry| {
                entry
                    .file_name()
                    .to_str()
                    .unwrap()
                    .starts_with("source.txt:")
            })
            .unwrap();
        assert!(moved_file.path().exists());
    }

    // Test nuke_file for an existing file
    #[test]
    fn test_nuke_file_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("delete.txt");
        File::create(&file_path).unwrap();
        let file_path_str = file_path.to_str().unwrap().to_string();

        nuke_file(&file_path_str);
        assert!(!file_path.exists());
    }

    // Test nuke_file for a non-existing file
    #[test]
    fn test_nuke_file_non_existing() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("nonexistent.txt");
        let file_path_str = file_path.to_str().unwrap().to_string();

        // Should not panic and handle error gracefully
        nuke_file(&file_path_str);
        assert!(!file_path.exists());
    }
}
