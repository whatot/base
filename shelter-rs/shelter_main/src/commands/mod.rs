use anyhow::Ok;
use clap::{ArgMatches, Command};
mod hello;
mod serve;

pub fn configure(command: Command) -> Command {
    command
        .subcommand(hello::configure())
        .subcommand(serve::configure())
}

pub fn handle(matches: &ArgMatches) -> anyhow::Result<()> {
    hello::handle(matches)?;
    serve::handle(matches)?;

    Ok(())
}
