use std::{fs, path::PathBuf};
use crate::{config::{model::Config, save}, utils::utils::resolve_path};
use anyhow::{Context, Result};


pub fn load(config_file: &Option<PathBuf>) -> Result<Config> {

    let path = resolve_path(config_file)?;

    if path.exists() {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file {:?}", path))?;
            
        let config : Config = toml::from_str(&content)
            .context("Invalid config file format")?;
            
        Ok(config)
    } else {
        let config = Config::default();
        save::save(&config, config_file)?;

        Ok(config)
    }

}
