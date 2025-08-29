use super::core::CLI;
use clap::{Arg, ArgAction, Command};

/// Creates a new CLI instance with all command line arguments configured
pub fn new() -> CLI {
    CLI{
    matches: Command::new("trashcan")
    .version(env!("CARGO_PKG_VERSION"))
        .author("Laurenz Schmidt")
        .about("A safer rm replacement with trashcan functionality")
        .long_about("A modern alternative to rm that moves files to a trash directory (~/.local/share/trashcan) instead of permanent deletion.")
        .after_help("Examples:\n  trashcan file.txt            # Move file to trash\n  trashcan --show-trashcan     # List trash contents\n  trashcan --restore           # Restore last deleted file\n  trashcan --nuke file.txt     # Permanently delete file")
        .arg(
            Arg::new("nuke")
                .long("nuke")
                .short('n')
                .help("Permanently delete files instead of moving to trash")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["remove-garbage", "show-trashcan", "restore"]),
        ) .arg(
            Arg::new("remove-garbage")
                .long("remove-garbage")
                .short('e')
                .help("Empty the entire trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["nuke", "show-trashcan", "restore"]),
        )
        .arg(
            Arg::new("show-trashcan")
                .long("show-trashcan")
                .short('l')
                .help("Show contents of the trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["nuke", "remove-garbage", "restore"]),
        )
        .arg(
            Arg::new("restore")
                .long("restore")
                .short('r')
                .help("Restore the last deleted file from trashcan")
                .action(ArgAction::SetTrue)
                .conflicts_with_all(["nuke", "remove-garbage", "show-trashcan"]),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Speak like a waterfall")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("interactive")
                .long("interactive")
                .short('i')
                .help("Prompt before each deletion")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("files")
                .help("Files or directories to operate on")
                .num_args(1..)
                .value_name("FILES")
                .required_unless_present_any(["remove-garbage", "show-trashcan", "restore"]),
        )
        .get_matches()
    }
}
