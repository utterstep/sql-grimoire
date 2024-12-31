use std::{ops::Deref, sync::Arc};

use axum_jwt_auth::{RemoteJwksDecoder, RemoteJwksDecoderBuilder};
use derive_getters::Getters;
use eyre::{Result, WrapErr};
use jsonwebtoken::{Algorithm, Validation};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPool;

use crate::config::Config;

#[derive(Clone)]
pub struct WebAppState(Arc<WebAppStateInner>);

#[derive(Getters)]
pub struct WebAppStateInner {
    db: PgPool,
    jwks_decoder: RemoteJwksDecoder,
    config: Config,
}

impl Deref for WebAppState {
    type Target = WebAppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl WebAppStateInner {
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

        let jwks_decoder = RemoteJwksDecoderBuilder::new(jwks_url.to_string())
            .with_validation(validation)
            .build();

        Ok(Self {
            db,
            jwks_decoder,
            config,
        })
    }
}

impl WebAppState {
    pub async fn from_config(config: Config) -> Result<Self, eyre::Report> {
        let inner = WebAppStateInner::new(config)
            .await
            .wrap_err("Failed to create app state")?;
        Ok(Self(Arc::new(inner)))
    }
}
