use std::path::Path;
use crate::trashcan::core::Trashcan;
use clap::{Arg, ArgAction, ArgGroup, Command};

const NUKE_FLAG: &str = "nuke";
const TRASHCAN_FLAG: &str = "trashcan";
const SHOW_TRASHCAN_FLAG: &str = "show-trashcan";
const FILES_ARG: &str = "files";

pub fn parse_args() -> clap::ArgMatches {
    Command::new("trashcan")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("rm replacement with safe deletion")
        .arg(
            Arg::new(NUKE_FLAG)
                .long(NUKE_FLAG)
                .help("Permanently delete files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(TRASHCAN_FLAG)
                .long(TRASHCAN_FLAG)
                .help("Clear the entire trashcan")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new(SHOW_TRASHCAN_FLAG)
                .long(SHOW_TRASHCAN_FLAG)
                .help("Show trashcan contents")
                .action(ArgAction::SetTrue),
        )
        .arg(Arg::new(FILES_ARG).help("Files to delete").num_args(1..))
        .group(
            ArgGroup::new("actions")
                .args([NUKE_FLAG, TRASHCAN_FLAG, SHOW_TRASHCAN_FLAG])
                .required(false),
        )
        .get_matches()
}

/// Verarbeitet Kommandozeilenargumente und führt die entsprechenden Aktionen aus.
pub fn check_flags(trashcan: &Trashcan, matches: clap::ArgMatches) -> Result<(), String> {
    if matches.get_flag(TRASHCAN_FLAG) {
        trashcan.nuke_trashcan_directory()?;
    } else if matches.get_flag(SHOW_TRASHCAN_FLAG) {
        trashcan.show_trashcan()?;
    } else if let Some(files) = matches.get_many::<String>(FILES_ARG) {
        //let nuke = matches.get_flag(NUKE_FLAG);
        for file in files {
            if !Path::new(file).exists() {
                //TODO:
                //return Err(|e: /* Type */| e.to_string());
            }
//            if nuke {
 //               file::nuke_file(file).map_err(|e| e.to_string())?;
  //          } else {
   //             file::move_file_to_trashcan(file, trashcan).map_err(|e| e.to_string())?;
    //        }
        }
    }
    Ok(())
}