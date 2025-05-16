use std::fs;


// hier noch mehr werte definieren, soll auch Umgebungsvarialben und .config gelesen werden
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
            println!("Trashcan directory: {}", self.location);
            fs::create_dir(self.location).unwrap();
        }
    }
}

