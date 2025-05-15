use std::fs;

pub struct Trashcan<'a> {
    pub location: &'a str,
    pub duration: u8,

}
impl Trashcan<'_> {
    pub fn check_trashcan(&self) {
        if !fs::exists(self.location).expect("Something went horribly wrong") {
            //TODO: Fehler behandeln
            #[cfg(debug_assertions)]
            println!("make trashcan");
            fs::create_dir(self.location).unwrap();
        }
    }
}

