use std::{ops::Deref, sync::Arc};

use derive_getters::Getters;
use secrecy::SecretString;
use serde::Deserialize;

use sql_grimoire_observability::Config as ObservabilityConfig;

#[derive(Debug, Deserialize, Getters)]
pub struct ConfigInner {
    #[serde(flatten)]
    observability: ObservabilityConfig,
    database_url: SecretString,
    bind_to: String,
    corbado_host: String,
    secret_key: SecretString,
}

#[derive(Clone)]
pub struct Config(Arc<ConfigInner>);

impl Deref for Config {
    type Target = ConfigInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        let config = envy::from_env::<ConfigInner>()?;
        Ok(Self(Arc::new(config)))
    }
}
