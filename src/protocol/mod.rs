use crate::protocol::becon::{BeconPayload, StatusPayload};

pub mod becon;
pub mod command;
pub mod status;

#[derive(Debug, Clone)]
pub enum Payload {
    Becon(BeconPayload),
    Status(StatusPayload),
}
