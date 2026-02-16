use crate::cli::args::{Cli, Command, PowerState};
use crate::config::load::load;
use crate::config::save;
use crate::device::command::CommandOptions;
use crate::device::discover::discover_devices;
use crate::device::target::{Target, resolve_target};
use crate::network::udp_server::atomber_udp_listener;
use crate::{device, protocol};

use anyhow::{Ok, Result};
use tokio::time::{Duration, sleep};

pub async fn run(cli: Cli) -> Result<()> {
    let mut config = load(&cli.config)?;

    match cli.command {
        Command::Discover(args) => {

            let mut rx = atomber_udp_listener(config.network.becon_port.clone()).await?;
            
            let device_results = discover_devices(
                &mut rx, 
                &config.devices, 
                Duration::from_secs(args.interval)
            ).await?;

            if args.save && device_results.changed {
                config.devices = device_results.devices;
                save::save(&config, &cli.config)?;
                println!("Config updated !")
            }
        }
        // TODO: Implement set, rename alias feature
        Command::Alias(args) => println!("{:#?}", args),

        // TODO: Implement create, add, remove, rename, delete group feautre 
        Command::Group(args) => println!("{:#?}", args),

        Command::Send(args) => {

            let command_options = CommandOptions{
                led: args.led.map(|state| matches!(state, PowerState::On)),
                power: args.power.map(|state| matches!(state, PowerState::On)),
                sleep: args.sleep.map(|state| matches!(state, PowerState::On)),
                speed: args.speed,
                timer: args.timer,
            };

            let mut rx = atomber_udp_listener(config.network.becon_port.clone()).await?;
            
            // TODO: replace _ with actual variable to be used with send command.
            let device_results = discover_devices(
                &mut rx, 
                &config.devices, 
                Duration::from_secs(1)
            ).await?;

            let target = if let Some(alias) = args.alias.as_deref()  {
                Target::Alias(alias)
            } else {
                Target::Group(args.group.as_deref().unwrap())
            };

            let device_list = resolve_target(target, &device_results.devices, &config.groups)?;

            for device in device_list {
                device::command::send(&device.ip, &config.network.command_port, &command_options).await?;
            }

            let timeout = sleep(Duration::from_secs(3));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    _ = &mut timeout => {
                        println!("Timeout waiting for status");
                        break;
                    }
                    Some(payload) = rx.recv() => {
                        if let protocol::Payload::Status(s) = payload {
                            println!("Status Received: {:?}",s);
                            break;
                        }
                    }
                }
            }
        }
    }

    // let debug_string = toml::to_string_pretty(&config)?;
    // println!("{}", debug_string);
    Ok(())
}
