// src/main.rs
mod cli;
mod trashcan;
mod utils;

use owo_colors::OwoColorize;
use std::process;

//TODO: shellautocompletion
fn main() {
    if let Err(e) = cli::core::start() {
        eprintln!("{} {}", "trashcan: ✗ Error -> ".red().bold(), e);
        process::exit(1);
    }
}
