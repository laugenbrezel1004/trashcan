// src/main.rs
mod cli;
mod trashcan;

use crate::trashcan::core::Trashcan;
use owo_colors::OwoColorize;
use std::process;

//TODO: shellautocompletion
fn main() {
    let trashcan = Trashcan::initialize()?;
    if let Err(e) = cli::core::start() {
        eprintln!("{} {}", "trashcan: ✗ Error -> ".red().bold(), e);
        process::exit(1);
    }
}
