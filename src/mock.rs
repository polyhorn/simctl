use super::{Device, DeviceQuery, Result, Simctl};

pub fn device() -> Result<Device> {
    Ok(Simctl::new()
        .list()?
        .devices()
        .iter()
        .available()
        .by_name("iPhone SE (2nd generation)")
        .next()
        .unwrap()
        .clone())
}
