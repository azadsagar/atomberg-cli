pub mod command;
pub mod discover;
pub mod send_workflow;
pub mod state_match;
pub mod target;

use std::collections::HashMap;

use crate::config::model::Device;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightMode {
    Warm,
    Cool,
    Daylight,
}

pub struct DeviceResults {
    pub devices: HashMap<String, Device>,
    pub changed: bool,
}

#[derive(Debug)]
pub struct DeviceState {
    pub power: bool,
    pub led: bool,
    pub sleep: bool,
    pub speed: u8,
    pub timer: u8,
    pub timer_elapsed_mins: u32,
    pub brightness: u8,
    pub light_mode: Option<LightMode>,
}
