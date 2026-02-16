use std::net::IpAddr;

use crate::protocol::becon::BeconPayload;
use crate::protocol::status::StatusPayload;

pub mod becon;
pub mod command;
pub mod status;

#[derive(Debug, Clone)]
pub enum Payload {
    Becon(BeconPayload),
    Status(StatusPayload),
}

pub fn parse_payload(buf: &[u8], src_ip: IpAddr) -> Option<Payload> {
    let content = std::str::from_utf8(buf).ok()?.trim();

    if content.len() < 12 {
        return None;
    }

    if content.len() <= 15 {
        becon::parse(content, src_ip).map(Payload::Becon)
    } else {
        status::parse(content).map(Payload::Status)
    }
}
