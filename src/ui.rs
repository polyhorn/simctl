//! Supporting types for the `simctl ui` subcommand.

use std::process::Stdio;

use super::{Device, Result, Validate};

/// Determines the appearance mode of the UI.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Appearance {
    /// Indicates a light appearance (i.e. the pre-iOS 13.0 default).
    Light,

    /// Indicates a dark appearance that was introduced in iOS 13.0.
    Dark,

    /// This is returned when trying to access the appearance of an unsupported
    /// device (e.g. watchOS or tvOS).
    Custom(String),
}

/// Wrapper around the `simctl ui` subcommand.
#[derive(Clone, Debug)]
pub struct UI {
    device: Device,
}

impl Device {
    /// Returns a wrapper around the `simctl ui` subcommand.
    pub fn ui(&self) -> UI {
        UI {
            device: self.clone(),
        }
    }
}

impl UI {
    /// Returns the current appearance of the UI of this device. Returns
    /// [`Appearance::Custom`] if the device doesn't support
    /// changing its appearance.
    pub fn appearance(&self) -> Result<Appearance> {
        let output = self
            .device
            .simctl()
            .command("ui")
            .arg(&self.device.udid)
            .arg("appearance")
            .stdout(Stdio::piped())
            .output()?;

        let output = output.validate_with_output()?;

        let appearance = String::from_utf8(output.stdout)?.trim().to_owned();
        Ok(match appearance.as_str() {
            "light" => Appearance::Light,
            "dark" => Appearance::Dark,
            _ => Appearance::Custom(appearance),
        })
    }

    /// Sets the current appearance of the UI of this device.
    pub fn set_appearance(&self, appearance: Appearance) -> Result<()> {
        let appearance = match &appearance {
            Appearance::Light => "light",
            Appearance::Dark => "dark",
            Appearance::Custom(appearance) => appearance,
        };

        self.device
            .simctl()
            .command("ui")
            .arg(&self.device.udid)
            .arg("appearance")
            .arg(appearance)
            .output()?
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
    fn test_appearance() -> Result<()> {
        mock::device()?.boot()?;

        mock::device()?.ui().set_appearance(Appearance::Dark)?;
        assert_eq!(mock::device()?.ui().appearance()?, Appearance::Dark);

        mock::device()?.ui().set_appearance(Appearance::Light)?;
        assert_eq!(mock::device()?.ui().appearance()?, Appearance::Light);

        mock::device()?.shutdown()?;

        Ok(())
    }
}
