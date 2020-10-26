//! Supporting types for the `simctl io` subcommand.

use std::process::Stdio;

use super::{Device, Result, Validate};

/// Distinguishes the display for devices that have multiple.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Display {
    /// Indicates the internal display. The internal display is the "main
    /// display" embedded in the hardware. This is supported by iOS and watchOS.
    Internal,

    /// Indicates the external display. The external display is a connected
    /// display. This is supported by the iOS simulator (although it's currently
    /// not possible to setup such an external display through this library) and
    /// tvOS, where it's the only available display because the hardware itself
    /// obviously doesn't have a display.
    External,
}

/// Controls the masking behavior that is used when taking a screenshot on
/// simulators of devices that feature rounded corners or a notch.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mask {
    /// Returns the unmasked frame buffer (without any masking applied).
    Ignored,

    /// Applies a mask to the alpha channel of a screenshot (i.e. the rounded
    /// corners and/or notch will be transparent).
    Alpha,

    /// Replaces colors outside of the mask with black (i.e. the rounded corners
    /// and/or notch will be black).
    Black,
}

/// Controls the encoding that will be used to write a screenshot to the buffer
/// that is returned.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ImageType {
    /// Returns a PNG-encoded image.
    Png,

    /// Returns a TIFF-encoded image.
    Tiff,

    /// Returns a BMP-encoded image.
    Bmp,

    /// Returns a GIF-encoded image.
    Gif,

    /// Returns a JPEG-encoded image.
    Jpeg,
}

/// Wrapper around the `simctl io` subcommand.
pub struct IO {
    device: Device,
}

impl Device {
    /// Returns a wrapper around the `simctl io` subcommand.
    pub fn io(&self) -> IO {
        IO {
            device: self.clone(),
        }
    }
}

impl IO {
    /// Takes a screenshot of the given display, with the given mask and returns
    /// a buffer of the image encoded using the given type.
    pub fn screenshot(
        &self,
        image_type: ImageType,
        display: Display,
        mask: Mask,
    ) -> Result<Vec<u8>> {
        let image_type = match image_type {
            ImageType::Png => "png",
            ImageType::Tiff => "tiff",
            ImageType::Bmp => "bmp",
            ImageType::Gif => "gif",
            ImageType::Jpeg => "jpeg",
        };

        let display = match display {
            Display::Internal => "internal",
            Display::External => "external",
        };

        let mask = match mask {
            Mask::Ignored => "ignored",
            Mask::Alpha => "alpha",
            Mask::Black => "black",
        };

        let output = self
            .device
            .simctl()
            .command("io")
            .arg(&self.device.udid)
            .arg("screenshot")
            .arg(format!("--type={}", image_type))
            .arg(format!("--display={}", display))
            .arg(format!("--mask={}", mask))
            .arg("-")
            .stdout(Stdio::piped())
            .output()?;

        let output = output.validate_with_output()?;

        Ok(output.stdout)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_screenshot() -> Result<()> {
        mock::device()?.boot()?;

        // The screenshot service often does not yet run immediately after
        // booting, so we might need to retry a couple of times.
        for i in 0..5 {
            match mock::device()?
                .io()
                .screenshot(ImageType::Png, Display::Internal, Mask::Ignored)
            {
                Ok(_) => break,
                Err(_) if i < 4 => continue,
                Err(error) => return Err(error),
            }
        }

        mock::device()?.shutdown()?;

        Ok(())
    }
}
