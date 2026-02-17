use anyhow::{Result, bail};

pub fn speed_parser(s: &str) -> Result<u8, &'static str> {
    let v: u8 = s.parse().map_err(|_| "Speed must be a number")?;

    if (1..=6).contains(&v) {
        Ok(v)
    } else {
        Err("Speed must be between 1 and 6")
    }
}

pub fn timer_parser(value: &str) -> Result<u8> {
    let parsed: u8 = value
        .parse()
        .map_err(|_| anyhow::anyhow!("Timer must be a number between 0 and 4"))?;

    if parsed > 4 {
        bail!("Timer must be between 0 and 4");
    }

    Ok(parsed)
}
