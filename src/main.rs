// src/main.rs
mod permissions; // Deklariert das Modul permissions (entspricht permissions.rs)

use std::env;
use std::fs;
use std::path::Path;
use nix::libc::{ftok, wait};
use nix::unistd;

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    let programmname = args[0].clone();
    check_trashcan();
    // Überspringe den ersten Argument (Programmname)
    for arg in args.iter().skip(1) {
        check_existence(&arg, &programmname);
    }
}

// Prüft, ob die Datei existiert
fn check_existence(argument: &str, programmname: &str)  {
    //if Path::new(argument).exists() {
    if fs::exists(argument).expect("Something went horribly wrong") {
        #[cfg(debug_assertions)]
        println!("removing {}", argument);
        if let Err(e) = move_file_to_trashcan(argument) {
            println!("Something went horribly wrong: {}", e);
        }
    }
    else {
        eprintln!("{}: cannot remove '\'{}\'': No such file or directory ",programmname , argument);
    }
}


fn move_file_to_trashcan(argument: &str) -> Result<(), String> {
    let uid = unistd::getuid();
    let destination = format!("/tmp/trashcan-{}", uid);
    fs::rename(argument, "/tmp").map_err(|e| e.to_string())?;
    nix::libc::rename()
    Ok(())
}

fn check_trashcan() {
    let uid = unistd::getuid();
    let dirname = format!("/tmp/trashcan-{}", uid);
    if !fs::exists(&dirname).expect("Something went horribly wrong") {
        //evtl noch den Fehler behandeln hahah
        #[cfg(debug_assertions)]
        println!("make trashcan");
        fs::create_dir(dirname).unwrap();
    }
}


