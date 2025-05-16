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

    for arg in args.iter().skip(1) {
        //check if files exists and delete if it does
       // flags::check_flags();
        file::check_existence(&arg, trashcan1.location);
    }
}
