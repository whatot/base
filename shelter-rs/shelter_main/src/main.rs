use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, Command};
use dotenv::dotenv;
use shelter_main::{commands, settings};
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, Registry};

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

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stdout));
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    commands::handle(&matches, &settings)?;

    Ok(())
}
