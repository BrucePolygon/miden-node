use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub const HOST: &str = "localhost";
// defined as: sum(ord(c)**p for (p, c) in enumerate('miden-store', 1)) % 2**16
pub const PORT: u16 = 28943;
pub const ENV_PREFIX: &str = "MIDEN_STORE";
pub const CONFIG_FILENAME: &str = "miden-store.toml";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct Endpoint {
    /// The host device the server will bind to.
    pub host: String,
    /// The port number to bind the server to.
    pub port: u16,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, Default)]
pub struct StoreConfig {
    /// Main endpoint of the server.
    pub endpoint: Endpoint,
}

impl Default for Endpoint {
    fn default() -> Self {
        Self {
            host: HOST.to_string(),
            port: PORT,
        }
    }
}

impl Display for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

impl miden_node_utils::Config for StoreConfig {
    const ENV_PREFIX: &'static str = ENV_PREFIX;
    const CONFIG_FILENAME: &'static str = CONFIG_FILENAME;
}