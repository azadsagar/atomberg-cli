use std::collections::HashMap;

use crate::cli::args::{Cli, Command};
use crate::config::load::load;
use crate::config::model::Device;
use crate::config::save;
use crate::network::udp_server::atomber_udp_listener;
use crate::protocol;

use anyhow::{Ok, Result};
use tokio::time::{Duration, sleep};

pub async fn run(cli: Cli) -> Result<()> {
    let mut config = load(&cli.config)?;

    match cli.command {
        Command::Discover(args) => {
            let mut rx = atomber_udp_listener(config.network.becon_port).await?;

            let timeout = sleep(Duration::from_secs(args.interval));

            tokio::pin!(timeout);

            println!("Discovering devices on network...");

            let mut devices: HashMap<String, Device> = config.devices.clone();
            let mut changed: bool = false;

            loop {
                tokio::select! {
                    _ = &mut timeout => break,
                    Some(payload) = rx.recv() => {
                        if let protocol::Payload::Becon(b) = payload {
                            //println!("Found device: {:?}", b);
                            let ip = b.ip.to_string();

                            match devices.get_mut(&b.device_id) {

                                Some(device) => {
                                    if device.ip != ip {
                                        device.ip = ip;
                                        changed = true;
                                    }
                                }

                                None => {
                                    devices.insert(
                                        b.device_id.clone(),
                                        Device {
                                            alias: Some(b.device_id.clone()),
                                            id: b.device_id.clone(),
                                            ip,
                                            groups: Vec::new(),
                                        }
                                    );

                                    changed = true;
                                }
                            }
                        }
                    }
                }
            }

            if args.save && changed {
                config.devices = devices;
                save::save(&config, &cli.config)?;
                println!("Config updated !")
            }
        }
        Command::Alias(args) => println!("{:#?}", args),
        Command::Group(args) => println!("{:#?}", args),
        Command::Send(args) => {
            let mut rx = atomber_udp_listener(config.network.becon_port).await?;

            // TODO: send command to smart fans
            // send_command(..).await ?

            println!("{:#?}", args);

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
