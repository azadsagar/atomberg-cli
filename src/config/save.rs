use std::{fs, path::PathBuf};

use crate::{config::model::Config, utils::utils::resolve_path};
use anyhow::{Context, Result};

pub fn save(confg: &Config, config_file: &Option<PathBuf>) -> Result<()> {
    let path = resolve_path(config_file)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create config directory {}", parent.display()))?;
    }

    let content = toml::to_string_pretty(confg).context("Failed to serialize config to toml")?;

    fs::write(&path, content)
        .with_context(|| format!("Failed to write config file {}", path.display()))?;

    Ok(())
}
