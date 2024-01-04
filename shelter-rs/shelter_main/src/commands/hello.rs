use anyhow::Ok;
use clap::{ArgMatches, Command};

pub fn configure() -> Command {
    Command::new("hello").about("Hello world")
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    if let Some(_match) = matches.subcommand_matches("hello") {
        println!("Hello world")
    }

    Ok(())
}
