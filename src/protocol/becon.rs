use std::net::IpAddr;

#[derive(Debug, Clone)]
pub struct BeconPayload {
    pub device_id: String, // 12 chars mac id is device id
    pub ip: IpAddr,
}

pub fn parse(content: &str, src_ip: IpAddr) -> Option<BeconPayload> {
    if content.len() < 12 {
        return None;
    }

    let mac = &content[..12];

    if !mac.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    Some(BeconPayload {
        device_id: mac.to_uppercase(),
        ip: src_ip,
    })
}
