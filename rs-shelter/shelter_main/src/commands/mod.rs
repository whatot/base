use anyhow::Ok;
use clap::{ArgMatches, Command};

use crate::settings::Settings;
mod create_admin;
mod hello;
mod migrate;
mod serve;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
        .subcommand(migrate::configure())
        .subcommand(create_admin::configure())
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    hello::handle(matches, settings)?;
    serve::handle(matches, settings)?;
    migrate::handle(matches, settings)?;
    create_admin::handle(matches, settings)?;

    Ok(())
}
