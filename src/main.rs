// src/main.rs

mod flags;
mod trashcan;

/*
TODO: Check file permission -> fehler ausgeben | wichtig wenn eigene implementation ohne "mv"
TODO:config file
TODO:umgebugnsvariablen?
TODO:mülleimer anzeigen -> typ anzeigen? größe vom eimer anzeigen?
TODO: letzte datei wiederherstellen
TODO: autocompletion in cmd
TODO: mv mit "guten" code ersetzen
*/

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
fn main() {

    let trashcan = match trashcan::core::initialize_trashcan() {
        Ok(trashcan) => trashcan,
        Err(e) => {
            eprintln!("trashcan error: {e}");
            std::process::exit(1);
        }
    };
    if let Err(e) = flags::core::check_flags(&trashcan, flags::core::parse_args()) {
        eprintln!("trashcan: cannot check flags {e}");
        std::process::exit(1);
    }
}
