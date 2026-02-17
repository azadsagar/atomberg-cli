use anyhow::{Ok, Result, bail};
use serde_json::{Map, Value};

#[derive(Debug, Default)]
pub struct CommandBuilder {
    map: Map<String, Value>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }

    pub fn power(mut self, on: bool) -> Self {
        self.map.insert("power".into(), Value::Bool(on));
        self
    }

    pub fn speed(mut self, speed: u8) -> Result<Self> {
        if !(1..=6).contains(&speed) {
            bail!("Speed must be between 1 and 6")
        }

        self.map.insert("speed".into(), Value::Number(speed.into()));
        Ok(self)
    }

    pub fn sleep(mut self, enabled: bool) -> Self {
        self.map.insert("sleep".into(), Value::Bool(enabled));
        self
    }

    pub fn timer(mut self, value: u8) -> Result<Self> {
        if !(0..=4).contains(&value) {
            bail!("Timer must be between 0 and 4")
        }

        self.map.insert("timer".into(), Value::Number(value.into()));
        Ok(self)
    }

    pub fn led(mut self, on: bool) -> Self {
        self.map.insert("led".into(), Value::Bool(on));
        self
    }

    pub fn build(self) -> Result<Vec<u8>> {
        if self.map.is_empty() {
            bail!("Failed to buld payload, no command specified.")
        }

        let json = Value::Object(self.map);

        //println!("{json}");

        Ok(serde_json::to_vec(&json)?)
    }
}
