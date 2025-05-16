// src/main.rs

mod file;
mod trashcan;

use clap::{Arg, Command};
//use file::{move_file_to_trashcan, check_existence};
use crate::file::{move_file_to_trashcan, nuke_file};
use std::env;
use std::process::exit;
use trashcan::Trashcan;
use users::os::unix::UserExt;
use users::{get_current_uid, get_user_by_uid};

fn main() {
    let args: Vec<String> = env::args().collect();
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    let mut location: String = "".to_string();
    // get home dir from user
    if let Some(user) = get_user_by_uid(get_current_uid()) {
        location = format!("{}/.local/share/trashcan", user.home_dir().display());
    }
    let trashcan1 = Trashcan {
        location: &location,
        duration: 10, //when to delete files -> in progress right now TODO:
    };

    // build trashcan if not available
    trashcan1.check_trashcan();

    let matches = Command::new("trashcan")
        .version("0.1.0")
        .author("Laurenz Schmidt")
        .about("rm replacement for dummies")
        .override_usage("trashcan [FLAGS] FileToDelete")
        .after_help(
            "ENVIRONMENT VARIABLES:\n\
Nothing... yet",
        )
        .arg(
            Arg::new("force")
                .long("force")
                .value_name("FILE")
                .help("destroy file forever and ever - without idiot protection")
                //.action(clap::ArgAction::SetTrue)
                .num_args(1..),
        )
        //        .arg(
        //            Arg::new("show_config")
        //                .short('s')
        //                .long("show-config")
        //                .help("Show the current config file")
        //                .action(clap::ArgAction::SetTrue),
        //        )
        .arg(
            Arg::new("files")
                .help("Files to delete")
                .num_args(1..) // Mindestens ein positionales Argument
                .index(1), // Positionale Argumente ab Index 2
        )
        .arg_required_else_help(true)
        .get_matches_from(env::args().collect::<Vec<String>>());

    // force delete a file if "force" is given
    if matches.contains_id("force") {
        // Zugriff auf alle Werte von --force
        if let Some(force_args) = matches.get_many::<String>("files") {
            if file::check_existence(&force_args., trashcan1.location) {
                move_file_to_trashcan(&arg, &trashcan1.location);
            }
        }
    }
    // wenn "normale" argumente vorhanden sind
    if let Some(files) = matches.get_many::<String>("files") {
        //damit "trashcan" nicht als file gewertet wird
        for arg in args.iter().skip(1) {
            //check if files exists and delete if it does
            #[cfg(debug_assertions)]
            println!("Files to delete:");
            if file::check_existence(&arg, trashcan1.location) {
                move_file_to_trashcan(&arg, &trashcan1.location);
            }
        }
    } else {
        unreachable!("clap sollte intervinieren");
        eprintln!("No files specified");
    }
}
