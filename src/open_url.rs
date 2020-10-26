use super::{Device, Result, Validate};

impl Device {
    /// Opens the given URL on this device.
    pub fn open_url(&self, path: &str) -> Result<()> {
        self.simctl()
            .command("openurl")
            .arg(&self.udid)
            .arg(path)
            .status()?
            .validate()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_open_url() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?.open_url("https://www.glacyr.com/")?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
