use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,

    #[serde(default)]
    pub devices: HashMap<String, Device>,

    #[serde(default)]
    pub groups: IndexMap<String, IndexSet<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub becon_port: u16,
    pub command_port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub alias: Option<String>,
    pub id: String,
    pub ip: String,
    pub groups: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            network: NetworkConfig {
                becon_port: 5625,
                command_port: 5600,
            },
            devices: HashMap::new(),
            groups: IndexMap::new(),
        }
    }
}
