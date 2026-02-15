use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct StatusPayload {
    pub device_id: String,
    pub raw: String,
    pub src_ip: IpAddr,
}

pub fn parse(content: &str, src_ip: IpAddr) -> Option<StatusPayload> {
    if content.len() < 12 {
        return None;
    }

    let mac = &content[..12];

    if !mac.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    Some(StatusPayload {
        device_id: mac.to_uppercase(),
        raw: content.to_string(),
        src_ip: src_ip,
    })
}
