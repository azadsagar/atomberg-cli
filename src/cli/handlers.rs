use crate::cli::args::{Cli, Command};
use crate::network::udp_server::atomber_udp_listener;
use crate::config::load::load;

use anyhow::{Ok, Result};
use tokio::{sync::oneshot, time::sleep};

pub async  fn run(cli: Cli) -> Result<()> {

    let config = load(&cli.config)?;

    match cli.command {
        Command::Discover(args) => {

            let (tx, rx) = oneshot::channel();
            let handle = tokio::spawn(async move {
                atomber_udp_listener(config.network.becon_port.clone(), rx).await;
            });
            sleep(std::time::Duration::from_secs(args.interval.clone())).await;

            let _ = tx.send(());
            let _ = handle.await;
        },
        Command::Alias(args) => println!("{:#?}", args),
        Command::Group(args) => println!("{:#?}", args),
        Command::Send(args) => println!("{:#?}", args),
    }

    // let debug_string = toml::to_string_pretty(&config)?;
    // println!("{}", debug_string);
    Ok(())
}
