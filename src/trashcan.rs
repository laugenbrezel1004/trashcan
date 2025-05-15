use std::fs;
use nix::unistd;

pub struct Trashcan {
    pub location: String,
    pub duration: u8,
    
}
impl Trashcan {
    pub fn check_trashcan(&self) {
        let uid = unistd::getuid();
        let dirname = format!("/tmp/trashcan-{}", uid);
        if !fs::exists(&dirname).expect("Something went horribly wrong") {
            //TODO: Fehler behandeln
            #[cfg(debug_assertions)]
            println!("make trashcan");
            fs::create_dir(dirname).unwrap();
        }
    } 
}

