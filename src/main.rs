// src/main.rs

mod trashcan;
mod file;
mod flags;
mod asdf;

use std::env;
use nix::unistd;
use trashcan::Trashcan;

fn main() {
    let args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    println!("args => {:?}", args);

    //get duration for deleting files
    let uid = unistd::getuid();
    let location = format!("/tmp/trashcan-{}", uid);
    let programname =  env::args().nth(0).unwrap();

    let trashcan1 = Trashcan{
        location: &location,
        duration: 10 //when to delete files -> in progress right now TODO:
    };

    // build trashcan if not available
    trashcan1.check_trashcan();

    for arg in args.iter().skip(1) {
        //check if files exists and delete if it does
        flags::check_flags();
        //file::check_existence(&arg, &programname, trashcan1.location)
    }
}
