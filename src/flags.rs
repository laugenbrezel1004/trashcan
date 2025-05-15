use std::cmp::min;
use clap::{Arg, Command};
use std::env;
use nix::libc::exit;

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
                .index(1), // Positionale Argumente ab Index 2
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
        // wenn keine files angegeben ist abbrachen
        let minanzahl : u8 = 2;
        if matches.get_count("files") < minanzahl {
            eprintln!("trashcan: missing operand");
            unsafe {
                exit(1);
            }
        }
        #[cfg(debug_assertions)]
        println!("Files to delete:");
        //damit "trashcan" nicht als file gewertet wird
        let new_files = files.skip(1);
        for file in new_files {
            #[cfg(debug_assertions)]
            println!("  - {}", file);
        }
    } else {
        println!("No files specified");
    }
}
