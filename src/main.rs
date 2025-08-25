
// src/main.rs
mod cli;
pub mod trashcan;

use cli::core::Cli;
use owo_colors::OwoColorize;
use std::process;

fn main() {
    if let Err(e) = Cli::new().run() {
        eprintln!("{} {}", "trashcan: ✗ Error -> ".red().bold(), e);
        process::exit(1);
    }
}
