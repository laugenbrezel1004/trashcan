use clap::{Command, Arg, ValueHint, value_parser, ArgAction};
use clap_complete::aot::{generate, Generator, Shell};
use std::io;

fn build_cli() -> Command {
    Command::new("example")
        .arg(Arg::new("file")
            .help("some input file")
            .value_hint(ValueHint::AnyPath))
        .arg(Arg::new("generator")
            .long("generate")
            .action(ArgAction::Set)
            .value_parser(value_parser!(Shell)))
}

fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, cmd.get_name().to_string(), &mut io::stdout());
}