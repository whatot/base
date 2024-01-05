use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use anyhow::Ok;
use clap::{value_parser, Arg, ArgMatches, Command};
use sea_orm::Database;
use tower_http::trace::TraceLayer;

use crate::{settings::Settings, state::ApplicationState};

pub fn configure() -> Command {
    Command::new("serve").about("Start HTTP Server").arg(
        Arg::new("port")
            .short('p')
            .long("port")
            .value_name("PORT")
            .help("TCP port to listen on")
            .default_value("8080")
            .value_parser(value_parser!(u16)),
    )
}

pub fn handle(matches: &ArgMatches, settings: &Settings) -> anyhow::Result<()> {
    if let Some(matches) = matches.subcommand_matches("serve") {
        let port: u16 = *matches.get_one("port").unwrap_or(&8080);

        start_tokio(port, settings)?;
    }

    Ok(())
}

fn start_tokio(port: u16, settings: &Settings) -> anyhow::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let db_url = settings.get_db_url();
            let db_conn = Database::connect(db_url)
                .await
                .expect("Database connection failed");

            let state = Arc::new(ApplicationState::new(settings, db_conn)?);

            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

            let routes = crate::api::configure(state).layer(TraceLayer::new_for_http());

            tracing::info!("starting axum on port {}", port);

            axum::Server::bind(&addr)
                .serve(routes.into_make_service())
                .await?;

            Ok(())
        })?;

    std::process::exit(0);
}
