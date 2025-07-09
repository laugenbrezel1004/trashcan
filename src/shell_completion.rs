use clap::{Command, Arg};
use clap_complete::{generate, shells, Shell};
use std::io;

pub fn build_cli() -> Command {
    Command::new("meinprogramm")
        .about("Beispiel für clap_completion")
        .subcommand(Command::new("completions")
            .about("Generiert Shell-Completion-Skripte")
            .arg(Arg::new("SHELL")
                .help("Die Ziel-Shell")
                .required(true)
                .value_parser(["bash", "zsh", "fish", "powershell", "elvish"])))
}