use std::collections::HashMap;

use crate::cli::args::{Cli, Command, PowerState};
use crate::config::load::load;
use crate::config::model::Device;
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

            let mut dev_status_validation: HashMap<String, Device> = HashMap::new();

            for device in device_list {
                device::command::send(&device.ip, &config.network.command_port, &command_options).await?;
                dev_status_validation.insert(device.id.clone(), device);
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
                        
                        if let protocol::Payload::Status(state) = payload {
                            if dev_status_validation.contains_key(&state.device_id.to_uppercase()) {

                                if state.message_id != "internet_query" && state.state_string.contains(',') {
                                    let first_field = state.state_string
                                        .split(',')
                                        .next()
                                        .ok_or_else(|| anyhow::anyhow!("Invalid State String"))?;

                                    let v: u32 = first_field.parse()?;

                                    let device_state = protocol::status::decode_device_state(v);

                                    let mut is_expected_state = true;

                                    if let Some(led_state) = args.led {
                                        let expected = matches!(led_state, PowerState::On);
                                        is_expected_state = is_expected_state && device_state.led == expected;
                                    }

                                    if let Some(power_state) = args.power {
                                        let expected = matches!(power_state, PowerState::On);
                                        is_expected_state = is_expected_state && device_state.power == expected;
                                    }

                                    if let Some(sleep_state) = args.sleep {
                                        let expected = matches!(sleep_state, PowerState::On);
                                        is_expected_state = is_expected_state && device_state.sleep == expected;
                                    }

                                    if let Some(speed) = args.speed {
                                        is_expected_state = is_expected_state && device_state.speed == speed;
                                    }

                                    if let Some(timer) = args.timer {
                                        is_expected_state = is_expected_state && device_state.timer == timer;
                                        println!("Timer elapsed mins is {}", &device_state.timer_elapsed_mins);
                                    }

                                    if is_expected_state {
                                        dev_status_validation.remove(&state.device_id.to_uppercase());
                                    }
                                }   
                            }
                        }

                        if dev_status_validation.is_empty(){
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
