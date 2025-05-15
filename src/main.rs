// src/main.rs

mod trashcan;

use std::env;
use std::fs;
use nix::unistd;
use std::process;
use trashcan::Trashcan;

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    let programmname = args[0].clone();
    let uid = unistd::getuid();
    let destination = format!("/tmp/trashcan-{}", uid);
    let trashcan1 = Trashcan{
        location: destination,
        duration: 10
    };
    trashcan1.check_trashcan();
    // Überspringe den ersten Argument (Programmname)
    for arg in args.iter().skip(1) {
        check_existence(&arg, &programmname);
    }
}

// Prüft, ob die Datei existiert
fn check_existence(argument: &str, programmname: &str)  {
    //if Path::new(argument).exists() {
    match fs::exists(argument) {
        Ok(true) => {
            if let Err(e) = move_file_to_trashcan(argument) {
                #[cfg(debug_assertions)]
                println!("removing {}", argument);
                println!("Something went horribly wrong: {}", e);
            }
        }
        Ok(false) | Err(_) => {
            eprintln!("{}: cannot remove '{}': No such file or directory", programmname, argument);
        }
    }
}


fn move_file_to_trashcan(argument: &str) -> Result<(), String> {
    //TODO: Fehler behandeln
    #[cfg(debug_assertions)]
    println!("destination => {:?}", destination);
    //fs::copy(argument, destination).map_err(|err| err.to_string())?;
    process::Command::new("mv")
        .arg(argument)
        .arg(destination)
        .output()
        .expect("failed to execute mv command");

    Ok(())
}




