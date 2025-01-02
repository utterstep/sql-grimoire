use apply::Apply;
use axum::Router;
use eyre::{Result, WrapErr};
use sqlx::migrate;
use tracing::info;

use crate::{config::Config, state::AppState};

mod logging;
mod routes;
mod security;

pub(crate) async fn setup_app(config: Config) -> Result<Router> {
    let app_state = AppState::from_config(config.clone()).await?;

    // TODO: some time in the future this should be separate, exactly-once routine
    info!("Running migrations");
    migrate!("../migrations")
        .run(app_state.db())
        .await
        .wrap_err("Migrations failed")?;
    info!("Successfully ran migrations");

    info!("Starting JWKS decoder");
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        app_state_clone
            .jwks_decoder()
            .refresh_keys_periodically()
            .await
    });

    info!("Setting up routes");
    let app = Router::new()
        // layers are applied in reverse order
        .apply(|app| routes::setup(app, app_state.clone()))
        .apply(|app| security::setup(app, &config))
        // logging should be added as a last layer, so that it can log all requests,
        // even those that fail in the middle of the stack
        .apply(logging::setup)
        .with_state(app_state);

    Ok(app)
}
