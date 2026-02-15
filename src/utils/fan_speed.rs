pub fn speed_parser(s: &str) -> Result<u8, &'static str> {
    let v: u8 = s.parse().map_err(|_| "Speed must be a number")?;

    if (1..=6).contains(&v) {
        Ok(v)
    } else {
        Err("Speed must be between 1 and 6")
    }
}
