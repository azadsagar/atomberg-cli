use std::net::IpAddr;


#[derive(Debug, Clone)]
pub struct BeconPayload {
    pub device_id: String, // 12 chars mac id is device id
    pub ip: IpAddr,
}

#[derive(Debug, Clone)]
pub struct StatusPayload {
    pub device_id: String,
    pub raw: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum Payload {
    Becon(BeconPayload),
    Status(StatusPayload),
}
