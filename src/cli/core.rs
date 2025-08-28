// src/cli.rs
use crate::trashcan::core::Trashcan;
use clap::ArgMatches;

pub struct CLI {
    pub matches: ArgMatches,
}

pub struct CLIMODES {
    pub interactive: bool,
    pub nuke: bool,
    pub verbose: bool,
}
/// Creates new clap CLI and call subfunction to check the given flags
pub fn start() -> Result<(), String> {
    // build new cli with parameters from the commandline
    let cli = super::build_cli::new();
    cli.manage()?;
    Ok(())
}
