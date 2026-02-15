use tokio::net::UdpSocket;
use tokio::sync::oneshot;

pub async fn atomber_udp_listener(port: u16, mut shutdown: oneshot::Receiver<()>) {
    let socket = UdpSocket::bind(("0.0.0.0", port))
        .await
        .expect("Failed to listen");

    let mut buf = [0u8; 4096];

    loop {
        tokio::select! {
            result = socket.recv_from(&mut buf) => {
                match result {
                    Ok((len, addr)) => {
                        println!("Received payload of size {}, from {}", len, addr);
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            _ = &mut shutdown => {
                break;
            }
        }
    }
}
