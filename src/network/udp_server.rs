use anyhow::Result;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

use crate::protocol;

pub async fn atomber_udp_listener(port: u16) -> Result<mpsc::Receiver<protocol::Payload>> {
    let socket = UdpSocket::bind(("0.0.0.0", port))
        .await
        .expect("Failed to listen");

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut buf = [0u8; 4096];

        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, src)) => {
                    if let Some(payload) = protocol::parse_payload(&buf[..len], src.ip()) {
                        if tx.send(payload).await.is_err() {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    });

    Ok(rx)
}
