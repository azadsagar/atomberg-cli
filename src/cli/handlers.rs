use crate::cli::args::{Cli, Command, PowerState, SendArgs};
use crate::config::load::load;
use crate::config::model::Config;
use crate::config::save;
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
        // TODO: Implement set, rename alias feature
        Command::Alias(args) => println!("{:#?}", args),

        // TODO: Implement create, add, remove, rename, delete group feautre
        Command::Group(args) => println!("{:#?}", args),

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
    }
}

fn requested_state_from_args(args: &SendArgs) -> RequestedState {
    RequestedState {
        led: args.led.map(|state| matches!(state, PowerState::On)),
        power: args.power.map(|state| matches!(state, PowerState::On)),
        sleep: args.sleep.map(|state| matches!(state, PowerState::On)),
        speed: args.speed,
        timer: args.timer,
    }
}
