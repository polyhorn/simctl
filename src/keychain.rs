//! Supporting types for the `simctl keychain` subcommand.

use super::{Device, Result, Validate};

/// Wrapper around the `simctl keychain` subcommand.
pub struct Keychain {
    device: Device,
}

impl Device {
    /// Returns an instance of the `keychain` subcommand.
    pub fn keychain(&self) -> Keychain {
        Keychain {
            device: self.clone(),
        }
    }
}

impl Keychain {
    /// Resets the device's keychain.
    pub fn reset(&self) -> Result<()> {
        self.device
            .simctl()
            .command("keychain")
            .arg(&self.device.udid)
            .arg("reset")
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
    fn test_keychain_reset() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?.keychain().reset()?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
