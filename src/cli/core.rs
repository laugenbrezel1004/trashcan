// src/cli.rs
use crate::trashcan::core::Trashcan;
use clap::ArgMatches;

pub struct CLI {
    pub matches: ArgMatches,
}

pub fn vprint(message: &str, verbose: bool) {
    if verbose {
        print!("{}", message);
    }
}
/// Creates new clap CLI and call subfunction to check the given flags
pub fn start() -> Result<(), String> {
    // build new cli with parameters from the commandline
    let cli = super::build_cli::new();
    cli.manage()?;
    Ok(())
}
