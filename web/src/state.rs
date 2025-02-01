use std::{ops::Deref, sync::Arc};

use axum_jwt_auth::{RemoteJwksDecoder, RemoteJwksDecoderBuilder};
use derive_getters::Getters;
use eyre::{Result, WrapErr};
use jsonwebtoken::{Algorithm, Validation};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState(Arc<AppStateInner>);

#[derive(Getters)]
pub struct AppStateInner {
    db: PgPool,
    jwks_decoder: RemoteJwksDecoder,
    config: Config,
}

impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AppStateInner {
    async fn new(config: Config) -> Result<Self, eyre::Report> {
        let db = PgPool::connect(config.database_url().expose_secret())
            .await
            .wrap_err("Failed to connect to database")?;
        let jwks_url = url::Url::parse(config.corbado_host())
            .wrap_err("Failed to parse corbado host")?
            .join("/.well-known/jwks")
            .wrap_err("Failed to join corbado host and jwks path")?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.algorithms = vec![Algorithm::RS256, Algorithm::RS384, Algorithm::RS512];

        let jwks_decoder = RemoteJwksDecoderBuilder::default()
            .jwks_url(jwks_url.to_string())
            .validation(validation)
            .config(Default::default())
            .keys_cache(Default::default())
            .client(Default::default())
            .build()
            .unwrap();

        Ok(Self {
            db,
            jwks_decoder,
            config,
        })
    }
}

impl AppState {
    pub async fn from_config(config: Config) -> Result<Self, eyre::Report> {
        let inner = AppStateInner::new(config)
            .await
            .wrap_err("Failed to create app state")?;
        Ok(Self(Arc::new(inner)))
    }
}
