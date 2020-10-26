use super::{Device, Result, Validate};

impl Device {
    /// Shuts down this device. Returns an error if it isn't booted.
    pub fn shutdown(&self) -> Result<()> {
        self.simctl()
            .command("shutdown")
            .arg(&self.info().udid)
            .output()?
            .validate()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::list::DeviceState;
    use crate::mock;

    #[test]
    #[serial]
    fn test_shutdown() -> Result<()> {
        mock::device()?.boot()?;
        assert_eq!(mock::device()?.state, DeviceState::Booted);

        mock::device()?.shutdown()?;
        assert_eq!(mock::device()?.state, DeviceState::Shutdown);

        Ok(())
    }
}
