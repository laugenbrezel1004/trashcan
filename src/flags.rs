use clap::{Arg, Command};
use std::env;

pub fn check_flags() {
    let matches = Command::new("trashcan")
        .version("0.1.0")
        .author("Laurenz Schmidt")
        .about("rm replacement for dummies")
        .after_help(
            "ENVIRONMENT VARIABLES:\n\
Nothing",
        )
        .arg(
            Arg::new("force")
                .long("force")
                .help("destroy file forever and ever - without idiot protection")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show_config")
                .short('s')
                .long("show-config")
                .help("Show the current config file")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches_from(env::args().collect::<Vec<String>>());
    // Hole die rohen Argumente (inklusive Subcommands)
    let raw_args: Vec<String> = std::env::args().collect();

    // Prüfe, ob verbose gesetzt ist
    if matches.get_flag("force") {
        println!("Forced flag present, will be overwritten");
    }
}
