use crate::device::{DeviceState, LightMode};

#[derive(Debug, Clone, Default)]
pub struct RequestedState {
    pub power: Option<bool>,
    pub speed: Option<u8>,
    pub sleep: Option<bool>,
    pub timer: Option<u8>,
    pub led: Option<bool>,
    pub brightness: Option<u8>,
    pub light_mode: Option<LightMode>,
}

pub fn matches_expected(actual: &DeviceState, expected: &RequestedState) -> bool {
    if let Some(power) = expected.power
        && actual.power != power
    {
        return false;
    }

    if let Some(speed) = expected.speed
        && actual.speed != speed
    {
        return false;
    }

    if let Some(sleep) = expected.sleep
        && actual.sleep != sleep
    {
        return false;
    }

    if let Some(timer) = expected.timer
        && actual.timer != timer
    {
        return false;
    }

    if let Some(led) = expected.led
        && actual.led != led
    {
        return false;
    }

    if let Some(brightness) = expected.brightness
        && actual.brightness != brightness
    {
        return false;
    }

    if let Some(light_mode) = expected.light_mode
        && actual.light_mode != Some(light_mode)
    {
        return false;
    }

    true
}
