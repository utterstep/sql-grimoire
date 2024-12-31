use eyre::WrapErr;
use tracing::info;

use sql_grimoire_observability::setup as setup_observability;

mod config;
mod db;
mod error;
mod extractors;
mod middlewares;
mod models;
mod partials;
mod routes;
mod setup;
mod state;
mod static_files;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), eyre::Report> {
    dotenvy::dotenv().ok();
    let config = Config::from_env().wrap_err("Failed to load config")?;

    setup_observability(config.observability())?;

    let app = setup::setup_app(config.clone()).await?;

    let addr = config.bind_to();
    info!(%addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
