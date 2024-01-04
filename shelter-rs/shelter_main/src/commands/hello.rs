use anyhow::Ok;
use clap::{ArgMatches, Command};

use crate::settings::Settings;

pub fn configure() -> Command {
    Command::new("hello").about("Hello world")
}

pub fn handle(matches: &ArgMatches, _settings: &Settings) -> anyhow::Result<()> {
    if let Some(_match) = matches.subcommand_matches("hello") {
        println!("Hello world")
    }

    Ok(())
}
