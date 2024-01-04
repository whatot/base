use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use dotenv::dotenv;
use shelter_main::commands;

pub fn main() -> Result<()> {
    dotenv().ok();

    let mut command = Command::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    command = commands::configure(command);

    let matches = command.get_matches();
    commands::handle(&matches)?;

    Ok(())
}
