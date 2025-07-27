// src/main.rs
mod cli;
mod trashcan;

use std::process;
use owo_colors::OwoColorize;
use cli::Cli;

fn main() {
    let cli = Cli::new();
    if let Err(e) = cli.run() {
        eprintln!("{} {}", "trashcan: ✗ Error -> ".red().bold(), e);
        process::exit(1);
    }
}