use serde::Deserialize;

use crate::device::{DeviceState, LightMode};

#[derive(Debug, Clone, Deserialize)]
pub struct StatusPayload {
    pub device_id: String,
    pub message_id: String,
    pub state_string: String,
}

pub fn parse(content: &str) -> Option<StatusPayload> {
    if content.len() < 12 {
        return None;
    }

    let raw = hex::decode(content).ok()?;

    let payload: StatusPayload = serde_json::from_slice(&raw).ok()?;

    Some(payload)
}

pub fn decode_device_state(state: u32) -> DeviceState {
    let power = (state & 0x10) > 0;
    let led = (state & 0x20) > 0;
    let sleep = (state & 0x80) > 0;
    let speed = (state & 0x07) as u8;
    let timer = ((state & 0x0F0000) >> 16) as u8;
    let timer_elapsed_mins = ((state & 0xFF000000) >> 24) * 4;
    let brightness = ((state & 0x7F00) >> 8) as u8;
    let cool = (state & 0x08) > 0;
    let warm = (state & 0x8000) > 0;
    let light_mode = match (cool, warm) {
        (true, true) => Some(LightMode::Daylight),
        (true, false) => Some(LightMode::Cool),
        (false, true) => Some(LightMode::Warm),
        (false, false) => None,
    };

    DeviceState {
        power,
        led,
        sleep,
        speed,
        timer,
        timer_elapsed_mins,
        brightness,
        light_mode,
    }
}
