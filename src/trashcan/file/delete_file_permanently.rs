use std::fs;
use std::path::Path;
use owo_colors::OwoColorize;

pub fn nuke_file( file: &str) -> Result<(), String> {
    let path = Path::new(file);
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| format!("Failed to delete directory: {e}"))?;
    } else {
        fs::remove_file(path).map_err(|e| format!("Failed to delete file: {e}"))?;

    }
    println!("{} {}", "💥 file {}".green().to_string(), file);
    
    Ok(())

}