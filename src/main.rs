use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use anyhow::{anyhow, Result};
use clap::Parser;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;
use backend_example::domain::user::User;

use backend_example::api::config_routes;
use backend_example::config;
use log::info;

lazy_static! {
    static ref HASHMAP: HashMap<String, User> = {
        let mut m = HashMap::new();
        let id = Uuid::new_v4();
        m.insert(id.to_string(),User {
            id,
            first_name: String::from("Corey"),
            last_name: String::from("Arnold"),
            url: String::from("test.com"),
        });
        m
    };
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {}

#[actix_web::main]
async fn main() -> Result<()> {
    // Parse CLI and load configuration.
    let _args = Args::parse();
    let mut cfg =
        config::Config::new().map_err(|err| anyhow!("Unable to read configuration: {err}"))?;

    env_logger::init();

    let mut users = HashMap::new();
    let id = "263496e3-7a97-4e9e-8215-55e1556a0cec";
    users.insert(id.to_string(),User {
        id: Uuid::from_str("263496e3-7a97-4e9e-8215-55e1556a0cec").unwrap(),
        first_name: String::from("Corey"),
        last_name: String::from("Arnold"),
        url: String::from("test.com"),
    });

    cfg.application.dev_users = Some(users);
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
