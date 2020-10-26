use std::ffi::OsStr;
use std::fmt::Display;

use super::{Device, Result, Validate};

impl Device {
    /// Boots this device. If the device is already booted, this function will
    /// return an error (as does the underlying CLI).
    ///
    /// NOTE: this does not automatically open the visual simulator interface.
    /// Use [`crate::Simctl::open()`] to open the visual interface.
    pub fn boot(&self) -> Result<()> {
        self.boot_with_env(Vec::<(String, &OsStr)>::new())
    }

    /// Boots this device with the given environment variables. Do not prepend
    /// `SIMCTL_CHILD_` to the variable names: this is done automatically. If
    /// the device is already booted, this function will return an error (as
    /// does the underlying CLI).
    ///
    /// NOTE: this does not automatically open the visual simulator interface.
    /// Use [`crate::Simctl::open()`] to open the visual interface.
    pub fn boot_with_env<I, K, V>(&self, envs: I) -> Result<()>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Display,
        V: AsRef<OsStr>,
    {
        self.simctl()
            .command("boot")
            .arg(&self.info().udid)
            .envs(
                envs.into_iter()
                    .map(|(key, value)| (format!("SIMCTL_CHILD_{}", key), value)),
            )
            .status()?
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
    fn test_boot() -> Result<()> {
        mock::device()?.boot()?;
        assert_eq!(mock::device()?.state, DeviceState::Booted);

        mock::device()?.shutdown()?;
        assert_eq!(mock::device()?.state, DeviceState::Shutdown);

        Ok(())
    }
}
