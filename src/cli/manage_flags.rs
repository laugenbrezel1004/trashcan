use crate::cli::core::CLI;
use crate::trashcan::core::Trashcan;

struct CliModes {
    interactive: bool,
    nuke: bool,
    verbose: bool,
}

enum OperationMode {
    RemoveGarbage,
    ShowTrashcan,
    Restore,
    HandleFiles,
}

impl CLI {
    /// Executes the appropriate action based on command line arguments
    pub fn manage(&self) -> Result<(), String> {
        let trashcanny = Trashcan::initialize()?;

        let modes = CliModes {
            interactive: self.matches.get_flag("interactive"),
            nuke: self.matches.get_flag("nuke"),
            verbose: self.matches.get_flag("verbose"),
        };

        // Determine the operation mode first
        let operation_mode = self.determine_operation_mode();

        // Execute the appropriate operation with the mode modifiers
        match operation_mode {
            OperationMode::RemoveGarbage => {
                trashcanny.remove_garbage(modes.interactive, modes.verbose)?
            }
            OperationMode::ShowTrashcan => trashcanny.list_contents(modes.verbose)?,
            OperationMode::Restore => trashcanny.restore(modes.verbose)?,
            OperationMode::HandleFiles => {
                self.handle_files(&trashcanny, modes.interactive, modes.nuke, modes.verbose)?
            }
        }

        Ok(())
    }

    /// Determines which primary operation to perform based on flags
    fn determine_operation_mode(&self) -> OperationMode {
        if self.matches.get_flag("remove_garbage") {
            OperationMode::RemoveGarbage
        } else if self.matches.get_flag("show_trashcan") {
            OperationMode::ShowTrashcan
        } else if self.matches.get_flag("restore") {
            OperationMode::Restore
        } else {
            OperationMode::HandleFiles
        }
    }
}
