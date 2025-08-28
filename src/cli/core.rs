// src/cli.rs
use clap::ArgMatches;

pub struct ClI {
    pub matches: ArgMatches,
}

pub struct CLIMODES {
    interactive: bool,
    nuke: bool,
    verbose: bool,
}
/// Creates new clap CLI and call subfunction to check the given flags
pub fn start() -> Result<(), String> {
    let cli = super::build_cli::new();
    cli.run()?;
    Ok(())
}
