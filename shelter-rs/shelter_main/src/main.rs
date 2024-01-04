use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use dotenv::dotenv;
use shelter_main::{commands, settings};

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

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "SHELTER")?;
    commands::handle(&matches, &settings)?;

    println!(
        "db url: {}",
        settings
            .database
            .url
            .unwrap_or("missing database url".to_string())
    );

    println!(
        "log level: {}",
        settings.logging.log_level.unwrap_or("info".to_string())
    );

    Ok(())
}
