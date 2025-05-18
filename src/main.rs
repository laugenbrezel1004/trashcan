// src/main.rs

mod file;
mod trashcan;

use crate::file::{move_file_to_trashcan, nuke_file};
use crate::trashcan::Trashcan;
use clap::{Arg, ArgAction, Command};
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

#[cfg(target_os = "linux")]
fn main() {
    let trashcan: Trashcan;
    // Trashcan initialisieren
    match initialize_trashcan() {
        Ok(trash) => {
            trashcan = trash;
        }
        // sollte nicht eintreffen, sollt
        Err(e) => {
            eprintln!("trashcan: Something went horribly wrong: {}", e);
            std::process::exit(1);
        }
    }
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
            if file::check_existence(&file) {
                if matches.get_flag("nuke") {
                    nuke_file(&file);
                } else if matches.get_flag("files") {
                    move_file_to_trashcan(&file, &trashcan.location);
                }
            } else {
                eprintln!("File not found: {}", file);
            }
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

#[cfg(test)]
mod tests {
    use crate::initialize_trashcan;


}
