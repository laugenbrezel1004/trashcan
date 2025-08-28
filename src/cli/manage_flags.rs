use crate::cli::core::CLI;
use crate::trashcan::core::Trashcan;

impl CLI {
    /// Executes the appropriate action based on command line arguments
    pub fn manage(&self) -> Result<(), String> {
        let trashcan = Trashcan::initialize()?;

        // check the  ok thingy later on i dont know if it can breakt somehing
        // if no flag, just delete the given files
        match (
            // replace these three with bools and give each function these bools
            self.matches.get_flag("interactive"),
            self.matches.get_flag("nuke"),
            self.matches.get_flag("verbose"),
            //stand alone flags
            self.matches.get_flag("remove_garbage"),
            self.matches.get_flag("show_trashcan"),
            self.matches.get_flag("restore"),
        ) {
            (true, _, _) => trashcan.remove_garbage()?,
            (_, _, _, true, _) => trashcan.list_contents()?,
            (_, _, false, false, true) => trashcan.restore()?,
            // just delete the given files
            _ => self.handle_files(&trashcan)?,
            // if there is no flag, delete the given files - if the user sumbmits no files, nor flags, clap should get the error
            // and prombt the user a error message
            // _ => println!(".?>"),
        }
        Ok(())
    }
}
