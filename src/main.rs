// src/main.rs

mod trashcan;
mod file;
mod flags;

use std::env;
use std::process::exit;
use nix::unistd;
use users::{get_user_by_uid, get_current_uid};
use users::os::unix::UserExt;
use trashcan::Trashcan;

fn main() {


    use std::cmp::min;
    use clap::{Arg, Command};
    use std::env;
    use nix::libc::exit;
    use crate::file;

    pub fn check_flags() {

        let args: Vec<String> = env::args().collect();
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
            #[cfg(debug_assertions)]
            println!("Files to delete:");
            file::check_existence(&arg, trashcan1.location);
            //damit "trashcan" nicht als file gewertet wird
            let new_files = files.skip(1);
            for file in new_files {
                #[cfg(debug_assertions)]
                println!("  - {}", file);
            }
        } else {
            eprintln!("No files specified");
        }
    }

    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    let mut location: String = "".to_string();

    //check if arguemnts are provide
    if args.len() < 2 {
        eprintln!("Too few arguments.");
        exit(1);
    }

    //get duration for deleting files
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        location = format!("{}/.local/share/trashcan", user.home_dir().display());
    }
    let trashcan1 = Trashcan{
        location: &location,
        duration: 10 //when to delete files -> in progress right now TODO:
    };

    // build trashcan if not available
    trashcan1.check_trashcan();

    //skip programname
    flags::check_flags();
    //for arg in args.iter().skip(1) {
        //check if files exists and delete if it does
    //}
}
