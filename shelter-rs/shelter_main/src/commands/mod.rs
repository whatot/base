use anyhow::Ok;
use clap::{ArgMatches, Command};

use crate::settings::Settings;
mod hello;
mod serve;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;

    Ok(())
}
