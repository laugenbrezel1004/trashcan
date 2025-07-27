// src/main.rs

mod flags;
use std::process;
mod trashcan;

#[cfg(target_os = "linux")]
#[cfg(target_arch = "x86_64")]
fn main() {
    let matches = flags::parse_args();
    let trashcan = match trashcan::initialize_trashcan() {
        Ok(trashcan) => trashcan,
        Err(e) => {
            eprintln!("trashcan: initialization failed: {}", e);
            process::exit(1);
        }
    };

    if let Err(e) = flags::process_flags(&trashcan, &matches) {
        eprintln!("trashcan: {}", e);
        process::exit(1);
    }
}