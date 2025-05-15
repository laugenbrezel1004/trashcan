// src/main.rs

mod trashcan;
mod file;

use std::env;
use nix::unistd;
use trashcan::Trashcan;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    #[cfg(debug_assertions)]
    println!("args => {:?}", args);
    
    let programname = args[0].clone();
    //get duration for deleting files
    let uid = unistd::getuid();
    let destination = format!("/tmp/trashcan-{}", uid);
    
    let trashcan1 = Trashcan{
        location: &destination,
        duration: 10 //when to delete files -> in progress right now
    };
    
    // build trashcan if not available
    trashcan1.check_trashcan();
    
    for arg in args.iter().skip(1) {
        // check if argument is valid
        if let Err(e) = file::check_existence(&arg, &programname) {
            Err(e) =>
            { 
                eprintln!("{}: cannot remove \'{}\': No such file or directory", &programname, &arg);
            }
        }
    }
}