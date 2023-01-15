use clap::{ Arg, Command,ArgAction, ArgMatches};
use anyhow::Result;

mod cli;
mod util;

const NAME: &str = env!("CARGO_PKG_NAME");

fn cli() -> Command{
        Command::new(NAME)
        .about("Tools for daily use.")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("generate")
                        .about("generate a secret")
                        .arg(
                            Arg::new("type")
                                .help("the type of generator to use")
                                .short('t')
                                .long("type")
                                .default_value("uuid++")
                                .value_parser(["lipsum", "random", "uuid", "uuid+", "uuid++", ]),
                        )
                        .arg(
                            Arg::new("length")
                                .help("the character length of secret to generate (ignored for fixed-length generator types)")
                                .short('l')
                                .long("length")
                                .value_parser(clap::value_parser!(usize))
                                .default_value("12"),
                        )
                        .arg(
                            Arg::new("suffix-length")
                                .help("the character length of a random suffix (for generator types that support suffixes)")
                                .long("suffix-length")
                                .value_parser(clap::value_parser!(usize))
                                .default_value("4"),
                        )
                        .arg(
                            Arg::new("word-count")
                                .help("the number of words to generate (for generator types that assemble words)")
                                .short('w')
                                .long("word-count")
                                .value_parser(clap::value_parser!(usize))
                                .default_value("4"),
                        )
                        .arg(
                            Arg::new("delimiter")
                                .help("the character used to join parts (for generator types that join parts)")
                                .short('d')
                                .long("delimiter")
                                .default_value("-"),
                        )
                        .arg(
                            Arg::new("encode")
                                .help("encode the generated password (uses base64)")
                                .short('e')
                                .long("encode")
                                .action(ArgAction::SetTrue),
                        ),
                )
}

fn run(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("generate", gen_matches)) => cli::gen::new(gen_matches)?,
        Some((&_, _)) => todo!(),
        None => todo!(),
    }
    Ok(())
}

fn main() -> Result<()> {
    let toolbox = cli();
    let matches = toolbox.clone().get_matches();
    run(&matches)
}