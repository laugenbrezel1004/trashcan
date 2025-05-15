use clap::{Arg, Command};
use std::env;

pub fn check_flags() {
    let matches = Command::new("trashcan")
        .version("0.1.0")
        .author("Laurenz Schmidt")
        .about("rm replacement for dummies")
        .override_usage("trashcan [FLAGS] FileToDelete")
        .after_help(
            "ENVIRONMENT VARIABLES:\n\
Nothing",
        )
        .arg(
            Arg::new("force")
                .long("force")
                .value_name("FILE")
                .help("destroy file forever and ever - without idiot protection")
                //.action(clap::ArgAction::SetTrue)
                .num_args(1..),
        )
        .arg(
            Arg::new("show_config")
                .short('s')
                .long("show-config")
                .help("Show the current config file")
               .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .help("Files to delete")
                .num_args(1..) // Mindestens ein positionales Argument
                .index(1), // Positionale Argumente ab Index 1
        )
        .get_matches_from(env::args().collect::<Vec<String>>());

    if matches.contains_id("force") {
        println!("Forced flag present, will be overwritten");
        // Zugriff auf alle Werte von --force
        if let Some(force_args) = matches.get_many::<String>("force") {
            #[cfg(debug_assertions)]
            println!("Force arguments:");
            for arg in force_args {
                println!("  - {}", arg);
            }
        } else {
            println!("trashcan: missing operand\nTry \'trashcan --help\' for more information.");
        }
    }
    if let Some(files) = matches.get_many::<String>("files") {
        #[cfg(debug_assertions)]
        println!("Files to delete:");
        for file in files {
            println!("  - {}", file);
        }
    } else {
        println!("No files specified");
    }
}
