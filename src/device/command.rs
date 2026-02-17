use std::net::IpAddr;

use crate::{network::udp_client::UdpClient, protocol::command::CommandBuilder};
use anyhow::Result;

pub struct CommandOptions {
    pub power: Option<bool>,
    pub speed: Option<u8>,
    pub sleep: Option<bool>,
    pub timer: Option<u8>,
    pub led: Option<bool>,
}

pub async fn send(ip: &IpAddr, port: &u16, opts: &CommandOptions) -> Result<()> {
    let mut builder = CommandBuilder::new();

    if let Some(p) = opts.power {
        builder = builder.power(p);
    }

    if let Some(s) = opts.speed {
        builder = builder.speed(s)?;
    }

    if let Some(enabled) = opts.sleep {
        builder = builder.sleep(enabled);
    }

    if let Some(t) = opts.timer {
        builder = builder.timer(t)?;
    }

    if let Some(l) = opts.led {
        builder = builder.led(l);
    }

    let payload = builder.build()?;

    let client = UdpClient::new().await?;
    client.send(ip, port, &payload).await?;

    Ok(())
}
