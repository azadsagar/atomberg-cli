use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use tokio::net::UdpSocket;

pub struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub async fn new() -> Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0").await?;
        Ok(Self { socket })
    }

    pub async fn send(&self, ip: &IpAddr, port: &u16, payload: &[u8]) -> Result<()> {
        let addr: SocketAddr = format!("{}:{port}", ip.to_string()).parse()?;
        self.socket.send_to(payload, addr).await?;
        Ok(())
    }
}
