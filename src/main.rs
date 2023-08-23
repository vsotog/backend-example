use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::{anyhow, Result};
use clap::Parser;

use backend_example::api::config_routes;
use backend_example::config;
use log::info;

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[actix_web::main]
async fn main() -> Result<()> {
    // Parse CLI and load configuration.
    let _args = Args::parse();
    let cfg =
        config::Config::new().map_err(|err| anyhow!("Unable to read configuration: {err}"))?;

    info!("config:\n{cfg}");

    let backend = backend_example::initialize_app(cfg.clone()).await?;

    let app_data = Data::new(backend);
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .configure(config_routes)
    })
    .bind(&cfg.application.listen_address)?
    .run()
    .await
    .map_err(|err| anyhow!(err.to_string()))
}
