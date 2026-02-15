use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,

    #[serde(default)]
    pub devices: HashMap<String, Device>,

    #[serde(default)]
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub becon_port: u16,
    pub command_port: u16,
    pub offline_after: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    pub alias: Option<String>,
    pub ip: String,
    pub last_seen: Option<DateTime<Utc>>,
    pub groups: Vec<String>,
    pub online: bool,
}



impl Default for Config {
    fn default() -> Self {
        Self { 
            network: NetworkConfig{
                becon_port: 5625,
                command_port: 5600,
                offline_after: 30,
            } ,
            devices: HashMap::new(), 
            groups: HashMap::new(), 
        }
    }
}
