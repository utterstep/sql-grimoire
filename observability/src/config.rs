use derive_getters::Getters;
use secrecy::SecretString;
use serde::Deserialize;
use url::Url;

fn two() -> usize {
    2
}

#[derive(Debug, Deserialize, Getters)]
pub struct Config {
    /// The environment in which the application is running.
    ///
    /// Multiple services can run in the same environment, i.e.
    /// web server, background worker, etc.
    environment: String,
    /// The number of spaces to indent trace tree output.
    #[serde(default = "two")]
    tree_indent_count: usize,
    /// The Honeycomb.io API key
    honeycomb_key: SecretString,
    /// The Honeycomb.io endpoint URL
    honeycomb_endpoint: Url,
}
