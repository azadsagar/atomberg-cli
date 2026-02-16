use std::collections::HashMap;
use indexmap::{IndexMap, IndexSet};
use crate::config::model::Device;

use anyhow::{Result, anyhow};

pub enum Target<'a> {
    Alias(&'a str),
    Group(&'a str),
}

pub fn resolve_target(
    target: Target,
    devices: &HashMap<String, Device>,
    groups: &IndexMap<String, IndexSet<String>>,
) -> Result<Vec<Device>> {
    match target {
        Target::Alias(alias) => resolve_alias(alias, devices),
        Target::Group(group) => resolve_group(group, devices, groups),
    }
}


fn resolve_alias(
    alias: &str,
    devices: &HashMap<String, Device>,
) -> anyhow::Result<Vec<Device>> {
    let device = devices
    .values()
    .find(|d| d.alias.as_deref() == Some(alias))
    .ok_or_else(|| anyhow!("Device ID '{}' not found", alias))?;

    Ok(vec![device.clone()])
}

fn resolve_group(
    group_name: &str,
    devices: &HashMap<String, Device>,
    groups: &IndexMap<String, IndexSet<String>>
) -> Result<Vec<Device>> {
    
    let entries = groups.get(group_name)
        .ok_or_else(|| anyhow!("Group '{}' not found", group_name))?;

    if entries.is_empty() {
        return Err(anyhow!("Group '{}' is empty", group_name));
    }

    let mut  resolved = Vec::with_capacity(entries.len());

    for entry in entries {
        
        if let Some(device) = devices.get(entry) {
            resolved.push(device.clone());
            continue;
        }

        if let Some(device) = devices.values().find(|d| d.alias.as_deref() == Some(entry)) {
            resolved.push(device.clone());
            continue;
        }

        return Err(anyhow!(
            "Entry '{}' in group '{}' does not match any device ID of alias",
            entry,
            group_name,
        ));

    }

    Ok(resolved)
}