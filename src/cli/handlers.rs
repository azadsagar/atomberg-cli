use crate::cli::args::{
    AliasArgs, AliasCommand, AliasShowArgs, Cli, Command, GroupArgs, GroupCommand,
    GroupMembershipArgs, LightModeState, PowerState, SendArgs,
};
use crate::config::load::load;
use crate::config::model::Config;
use crate::config::ops::{self, DeviceSelector};
use crate::config::save;
use crate::device::LightMode;
use crate::device::command::CommandOptions;
use crate::device::discover::discover_devices;
use crate::device::send_workflow;
use crate::device::state_match::RequestedState;
use crate::device::target::{Target, resolve_target};
use crate::network::udp_server::atomber_udp_listener;

use anyhow::Result;
use tokio::time::Duration;

pub async fn run(cli: Cli) -> Result<()> {
    let mut config = load(&cli.config)?;

    match cli.command {
        Command::Discover(args) => {
            let mut rx = atomber_udp_listener(config.network.becon_port).await?;

            let device_results =
                discover_devices(&mut rx, &config.devices, Duration::from_secs(args.interval))
                    .await?;

            if args.save && device_results.changed {
                config.devices = device_results.devices;
                save::save(&config, &cli.config)?;
                println!("Config updated !")
            }
        }
        Command::Alias(args) => {
            if handle_alias(args, &mut config)? {
                save::save(&config, &cli.config)?;
            }
        }

        Command::Group(args) => {
            if handle_group(args, &mut config)? {
                save::save(&config, &cli.config)?;
            }
        }

        Command::Send(args) => handle_send(args, &config).await?,
    }

    Ok(())
}

async fn handle_send(args: SendArgs, config: &Config) -> Result<()> {
    let command_options = command_options_from_args(&args);
    let expected_state = requested_state_from_args(&args);

    let mut rx = atomber_udp_listener(config.network.becon_port).await?;

    let device_results = discover_devices(&mut rx, &config.devices, Duration::from_secs(1)).await?;

    let target = if let Some(alias) = args.alias.as_deref() {
        Target::Alias(alias)
    } else {
        Target::Group(args.group.as_deref().unwrap())
    };

    let device_list = resolve_target(target, &device_results.devices, &config.groups)?;

    let result = send_workflow::send_and_wait_for_expected_state(
        &mut rx,
        device_list,
        &config.network.command_port,
        &command_options,
        &expected_state,
        Duration::from_secs(3),
    )
    .await?;

    if result.timed_out {
        println!("Timeout waiting for status");
    }

    Ok(())
}

fn command_options_from_args(args: &SendArgs) -> CommandOptions {
    CommandOptions {
        led: args.led.map(|state| matches!(state, PowerState::On)),
        power: args.power.map(|state| matches!(state, PowerState::On)),
        sleep: args.sleep.map(|state| matches!(state, PowerState::On)),
        speed: args.speed,
        timer: args.timer,
        brightness: args.set_brightness,
        light_mode: parse_light_mode(args),
    }
}

fn requested_state_from_args(args: &SendArgs) -> RequestedState {
    RequestedState {
        led: args.led.map(|state| matches!(state, PowerState::On)),
        power: args.power.map(|state| matches!(state, PowerState::On)),
        sleep: args.sleep.map(|state| matches!(state, PowerState::On)),
        speed: args.speed,
        timer: args.timer,
        brightness: args.set_brightness,
        light_mode: parse_light_mode(args),
    }
}

fn parse_light_mode(args: &SendArgs) -> Option<LightMode> {
    args.color.map(|color| match color {
        LightModeState::Warm => LightMode::Warm,
        LightModeState::Cool => LightMode::Cool,
        LightModeState::Daylight => LightMode::Daylight,
    })
}

fn handle_alias(args: AliasArgs, config: &mut Config) -> Result<bool> {
    match args.command {
        AliasCommand::Set { device_id, alias } => {
            let changed = ops::set_alias(config, &device_id, &alias)?;
            if changed {
                println!(
                    "Alias '{}' set for device '{}'",
                    alias,
                    device_id.to_uppercase()
                );
            } else {
                println!("Alias unchanged");
            }
            Ok(changed)
        }
        AliasCommand::Rename { old, new } => {
            let changed = ops::rename_alias(config, &old, &new)?;
            if changed {
                println!("Alias renamed from '{}' to '{}'", old, new);
            } else {
                println!("Alias unchanged");
            }
            Ok(changed)
        }
        AliasCommand::Remove { alias } => {
            let changed = ops::remove_alias(config, &alias)?;
            if changed {
                println!("Alias '{}' removed and replaced with device ID", alias);
            } else {
                println!("Alias unchanged");
            }
            Ok(changed)
        }
        AliasCommand::List => {
            let mut devices: Vec<_> = config.devices.values().collect();
            devices.sort_by(|a, b| a.id.cmp(&b.id));

            for device in devices {
                println!(
                    "{} -> {}",
                    device.id,
                    device.alias.as_deref().unwrap_or("<none>")
                );
            }
            Ok(false)
        }
        AliasCommand::Show(selector) => {
            let device = resolve_alias_show(config, selector)?;
            println!("Device ID: {}", device.id);
            println!("Alias: {}", device.alias.as_deref().unwrap_or("<none>"));
            if device.groups.is_empty() {
                println!("Groups: <none>");
            } else {
                println!("Groups: {}", device.groups.join(", "));
            }
            Ok(false)
        }
    }
}

fn handle_group(args: GroupArgs, config: &mut Config) -> Result<bool> {
    match args.command {
        GroupCommand::Create { name } => {
            let changed = ops::create_group(config, &name)?;
            if changed {
                println!("Group '{}' created", name);
            }
            Ok(changed)
        }
        GroupCommand::Rename { old, new } => {
            let changed = ops::rename_group(config, &old, &new)?;
            if changed {
                println!("Group '{}' renamed to '{}'", old, new);
            } else {
                println!("Group unchanged");
            }
            Ok(changed)
        }
        GroupCommand::Delete { name } => {
            let changed = ops::delete_group(config, &name)?;
            if changed {
                println!("Group '{}' deleted", name);
            }
            Ok(changed)
        }
        GroupCommand::Add(args) => {
            let selector = resolve_membership_selector(&args);
            let changed = ops::add_group_member(config, &args.name, selector)?;
            if changed {
                println!("Device added to group '{}'", args.name);
            } else {
                println!("Group membership unchanged");
            }
            Ok(changed)
        }
        GroupCommand::Remove(args) => {
            let selector = resolve_membership_selector(&args);
            let changed = ops::remove_group_member(config, &args.name, selector)?;
            if changed {
                println!("Device removed from group '{}'", args.name);
            } else {
                println!("Group membership unchanged");
            }
            Ok(changed)
        }
        GroupCommand::List => {
            for (name, members) in &config.groups {
                println!(
                    "{} ({} member{})",
                    name,
                    members.len(),
                    if members.len() == 1 { "" } else { "s" }
                );
            }
            Ok(false)
        }
        GroupCommand::Show { name } => {
            let members = config
                .groups
                .get(&name)
                .ok_or_else(|| anyhow::anyhow!("Group '{}' not found", name))?;
            println!("Group: {}", name);
            for member in members {
                println!("{}", member);
            }
            Ok(false)
        }
    }
}

fn resolve_alias_show(
    config: &Config,
    args: AliasShowArgs,
) -> Result<crate::config::model::Device> {
    let selector = if let Some(device_id) = args.device_id.as_deref() {
        DeviceSelector::DeviceId(device_id)
    } else {
        DeviceSelector::Alias(args.alias.as_deref().unwrap())
    };
    ops::resolve_device(config, selector)
}

fn resolve_membership_selector(args: &GroupMembershipArgs) -> DeviceSelector<'_> {
    if let Some(device_id) = args.device_id.as_deref() {
        DeviceSelector::DeviceId(device_id)
    } else {
        DeviceSelector::Alias(args.alias.as_deref().unwrap())
    }
}
