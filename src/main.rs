// src/main.rs
mod permissions; // Deklariert das Modul permissions (entspricht permissions.rs)

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    let programmname = args[0].clone();

    // Überspringe den ersten Argument (Programmname)
    for arg in args.iter().skip(1) {
        check_existence(&arg, &programmname);
    }
}

// Prüft, ob die Datei existiert
fn check_existence(argument: &str, programmname: &str)  {
    if Path::new(argument).exists() {
            println!("removing {}", argument);
        fs::remove_file(argument).unwrap();
    }
    else {
        eprintln!("{}: cannot remove '\'{}\'': No such file or directory ",programmname , argument);
    }
}


fn delete_file(argument: &str) -> Result<(), String> {
    fs::remove_file(argument).map_err(|e| e.to_string())?;
    Ok(())
}


