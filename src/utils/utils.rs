use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn default_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;

    Ok(home.join(".atomberg").join("config"))
}

pub fn resolve_path(path: &Option<PathBuf>) -> Result<PathBuf> {
    match path {
        Some(p) => Ok(p.clone()),
        None => default_config_path(),
    }
}
