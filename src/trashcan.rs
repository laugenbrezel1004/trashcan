use std::fs;
use std::process;

// hier noch mehr werte definieren, soll auch Umgebungsvarialben und .config gelesen werden
pub struct Trashcan<'a> {
    pub location: &'a str,
    pub duration: u8,
}
impl Trashcan<'_> {
    pub fn make_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("make trashcan");
        if let Err(e) = fs::create_dir(self.location) {
            eprintln!("trashcan: cant create trashcan directory -> {}", e);
            eprintln!("You should check your trashcan directory and make sure that it can be created at valid point");
            process::exit(1);
        }
    }

    pub fn clear_trashcan(&self) {
        #[cfg(debug_assertions)]
        println!("nuke trashcan directory");
        if let Err(e) = fs::remove_dir_all(self.location) {
            eprintln!("trashcan: cannot clean trashcan directory-> {}", e);
            eprintln!("You should check your trashcan directory and make sure that it can be created at valid point");
            process::exit(1);
        }
    }
}
