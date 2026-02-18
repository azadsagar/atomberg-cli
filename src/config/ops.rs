use crate::config::model::{Config, Device};
use anyhow::{Result, anyhow};

pub enum DeviceSelector<'a> {
    DeviceId(&'a str),
    Alias(&'a str),
}

pub fn set_alias(config: &mut Config, device_id: &str, alias: &str) -> Result<bool> {
    let device_id = normalize_device_id(device_id);
    let alias = normalize_alias(alias)?;

    let existing_owner = find_device_id_by_alias(config, &alias);
    if existing_owner.as_deref() != Some(device_id.as_str()) && existing_owner.is_some() {
        return Err(anyhow!(
            "Alias '{}' is already used by another device",
            alias
        ));
    }

    let device = config
        .devices
        .get_mut(&device_id)
        .ok_or_else(|| anyhow!("Device ID '{}' not found", device_id))?;

    let old_alias = device.alias.clone();
    if old_alias.as_deref() == Some(alias.as_str()) {
        return Ok(false);
    }

    device.alias = Some(alias.clone());
    remap_group_member_entries(config, old_alias.as_deref(), &alias);

    Ok(true)
}

pub fn rename_alias(config: &mut Config, old: &str, new: &str) -> Result<bool> {
    let old = normalize_alias(old)?;
    let new = normalize_alias(new)?;

    let device_id = find_device_id_by_alias(config, &old)
        .ok_or_else(|| anyhow!("Alias '{}' not found", old))?;

    set_alias(config, &device_id, &new)
}

pub fn remove_alias(config: &mut Config, alias: &str) -> Result<bool> {
    let alias = normalize_alias(alias)?;

    let device_id = find_device_id_by_alias(config, &alias)
        .ok_or_else(|| anyhow!("Alias '{}' not found", alias))?;

    let replacement_alias = config
        .devices
        .get(&device_id)
        .map(|d| d.id.clone())
        .ok_or_else(|| anyhow!("Device ID '{}' not found", device_id))?;

    set_alias(config, &device_id, &replacement_alias)
}

pub fn create_group(config: &mut Config, name: &str) -> Result<bool> {
    let name = normalize_group_name(name)?;
    if config.groups.contains_key(&name) {
        return Err(anyhow!("Group '{}' already exists", name));
    }

    config.groups.insert(name, Default::default());
    Ok(true)
}

pub fn rename_group(config: &mut Config, old: &str, new: &str) -> Result<bool> {
    let old = normalize_group_name(old)?;
    let new = normalize_group_name(new)?;

    if old == new {
        return Ok(false);
    }

    if config.groups.contains_key(&new) {
        return Err(anyhow!("Group '{}' already exists", new));
    }

    let members = config
        .groups
        .shift_remove(&old)
        .ok_or_else(|| anyhow!("Group '{}' not found", old))?;

    config.groups.insert(new.clone(), members);

    for device in config.devices.values_mut() {
        for group in &mut device.groups {
            if group == &old {
                *group = new.clone();
            }
        }
        device.groups.sort();
        device.groups.dedup();
    }

    Ok(true)
}

pub fn delete_group(config: &mut Config, name: &str) -> Result<bool> {
    let name = normalize_group_name(name)?;

    let existed = config
        .groups
        .shift_remove(&name)
        .ok_or_else(|| anyhow!("Group '{}' not found", name))?;

    if existed.is_empty() {
        for device in config.devices.values_mut() {
            device.groups.retain(|g| g != &name);
        }
        return Ok(true);
    }

    for device in config.devices.values_mut() {
        device.groups.retain(|g| g != &name);
    }

    Ok(true)
}

pub fn add_group_member(
    config: &mut Config,
    group_name: &str,
    selector: DeviceSelector<'_>,
) -> Result<bool> {
    let group_name = normalize_group_name(group_name)?;
    let device_id = resolve_device_id(config, selector)?;

    let mut changed = false;
    {
        let members = config
            .groups
            .get_mut(&group_name)
            .ok_or_else(|| anyhow!("Group '{}' not found", group_name))?;

        if members.insert(device_id.clone()) {
            changed = true;
        }
    }

    let device = config
        .devices
        .get_mut(&device_id)
        .ok_or_else(|| anyhow!("Device ID '{}' not found", device_id))?;

    if !device.groups.iter().any(|g| g == &group_name) {
        device.groups.push(group_name);
        changed = true;
    }

    Ok(changed)
}

pub fn remove_group_member(
    config: &mut Config,
    group_name: &str,
    selector: DeviceSelector<'_>,
) -> Result<bool> {
    let group_name = normalize_group_name(group_name)?;
    let device_id = resolve_device_id(config, selector)?;

    let mut changed = false;
    {
        let members = config
            .groups
            .get_mut(&group_name)
            .ok_or_else(|| anyhow!("Group '{}' not found", group_name))?;

        if members.shift_remove(&device_id) {
            changed = true;
        }
    }

    let device = config
        .devices
        .get_mut(&device_id)
        .ok_or_else(|| anyhow!("Device ID '{}' not found", device_id))?;

    let previous_len = device.groups.len();
    device.groups.retain(|g| g != &group_name);
    if device.groups.len() != previous_len {
        changed = true;
    }

    Ok(changed)
}

pub fn resolve_device(config: &Config, selector: DeviceSelector<'_>) -> Result<Device> {
    let device_id = resolve_device_id(config, selector)?;
    config
        .devices
        .get(&device_id)
        .cloned()
        .ok_or_else(|| anyhow!("Device ID '{}' not found", device_id))
}

fn resolve_device_id(config: &Config, selector: DeviceSelector<'_>) -> Result<String> {
    match selector {
        DeviceSelector::DeviceId(device_id) => {
            let normalized = normalize_device_id(device_id);
            if !config.devices.contains_key(&normalized) {
                return Err(anyhow!("Device ID '{}' not found", normalized));
            }
            Ok(normalized)
        }
        DeviceSelector::Alias(alias) => {
            let alias = normalize_alias(alias)?;
            find_device_id_by_alias(config, &alias)
                .ok_or_else(|| anyhow!("Alias '{}' not found", alias))
        }
    }
}

fn remap_group_member_entries(config: &mut Config, old_alias: Option<&str>, new_alias: &str) {
    let Some(old_alias) = old_alias else {
        return;
    };

    if old_alias == new_alias {
        return;
    }

    for members in config.groups.values_mut() {
        if members.shift_remove(old_alias) {
            members.insert(new_alias.to_string());
        }
    }
}

fn normalize_device_id(device_id: &str) -> String {
    device_id.trim().to_uppercase()
}

fn normalize_alias(alias: &str) -> Result<String> {
    let alias = alias.trim();
    if alias.is_empty() {
        return Err(anyhow!("Alias must not be empty"));
    }
    Ok(alias.to_string())
}

fn normalize_group_name(name: &str) -> Result<String> {
    let name = name.trim();
    if name.is_empty() {
        return Err(anyhow!("Group name must not be empty"));
    }
    Ok(name.to_string())
}

fn find_device_id_by_alias(config: &Config, alias: &str) -> Option<String> {
    config.devices.iter().find_map(|(device_id, device)| {
        (device.alias.as_deref() == Some(alias)).then(|| device_id.clone())
    })
}
