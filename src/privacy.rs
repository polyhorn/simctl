//! Supporting types for the `simctl privacy` subcommand.

use super::{Device, Result, Validate};

/// Refers to a specific service that an app needs to have permission for to
/// access.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PrivacyService {
    /// Wildcard that includes all services.
    All,

    /// Grants access to a user's calendar.
    Calendar,

    /// Grants limited access to a user's contacts.
    ContactsLimited,

    /// Grants access to a user's contacts.
    Contacts,

    /// Grants access to a user's location when an app is active.
    Location,

    /// Grants access to a user's location even if the app is in background.
    LocationAlways,

    /// Grants access to adding photos to the user's photo library.
    PhotosAdd,

    /// Grants access to reading photos from the user's photo library.
    Photos,

    /// Grants access to the user's media library (i.e. music and videos).
    MediaLibrary,

    /// Grants access to the user's microphone (which will most likely be the
    /// microphone of the Mac that the simulator runs on).
    Microphone,

    /// Grants access to the user's motion sensors.
    Motion,

    /// Grants access to the user's reminders.
    Reminders,

    /// Grants access to Siri.
    Siri,
}

impl ToString for PrivacyService {
    fn to_string(&self) -> String {
        match self {
            PrivacyService::All => "all",
            PrivacyService::Calendar => "calendar",
            PrivacyService::ContactsLimited => "contacts-limited",
            PrivacyService::Contacts => "contacts",
            PrivacyService::Location => "location",
            PrivacyService::LocationAlways => "location-always",
            PrivacyService::PhotosAdd => "photos-add",
            PrivacyService::Photos => "photos",
            PrivacyService::MediaLibrary => "media-library",
            PrivacyService::Microphone => "microphone",
            PrivacyService::Motion => "motion",
            PrivacyService::Reminders => "reminders",
            PrivacyService::Siri => "siri",
        }
        .to_owned()
    }
}

/// Wrapper around the `simctl privacy` subcommand.
pub struct Privacy {
    device: Device,
}

impl Device {
    /// Returns a wrapper around the `simctl privacy` subcommand.
    pub fn privacy(&self) -> Privacy {
        Privacy {
            device: self.clone(),
        }
    }
}

impl Privacy {
    /// Grants access to the given service to an application with the given
    /// bundle ID.
    pub fn grant(&self, service: PrivacyService, bundle_id: &str) -> Result<()> {
        self.device
            .simctl()
            .command("privacy")
            .arg(&self.device.udid)
            .arg("grant")
            .arg(service.to_string())
            .arg(bundle_id)
            .status()?
            .validate()
    }

    /// Revokes access to the given service from an application with the given
    /// bundle ID.
    pub fn revoke(&self, service: PrivacyService, bundle_id: &str) -> Result<()> {
        self.device
            .simctl()
            .command("privacy")
            .arg(&self.device.udid)
            .arg("revoke")
            .arg(service.to_string())
            .arg(bundle_id)
            .status()?
            .validate()
    }

    /// Resets access to the given service from an application with the given
    /// bundle ID. This will cause the OS to ask again when this app requests
    /// permission to use the given service.
    pub fn reset(&self, service: PrivacyService, bundle_id: &str) -> Result<()> {
        self.device
            .simctl()
            .command("privacy")
            .arg(&self.device.udid)
            .arg("reset")
            .arg(service.to_string())
            .arg(bundle_id)
            .status()?
            .validate()
    }

    /// Resets access to the given service from all applications running on the
    /// device.
    pub fn reset_all(&self, service: PrivacyService) -> Result<()> {
        self.device
            .simctl()
            .command("privacy")
            .arg(&self.device.udid)
            .arg("reset")
            .arg(service.to_string())
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
    fn test_privacy_grant() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?
            .privacy()
            .grant(PrivacyService::Location, "com.apple.Maps")?;
        mock::device()?.shutdown()?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_privacy_revoke() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?
            .privacy()
            .revoke(PrivacyService::Location, "com.apple.Maps")?;
        mock::device()?.shutdown()?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_privacy_reset() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?
            .privacy()
            .grant(PrivacyService::Location, "com.apple.Maps")?;
        mock::device()?
            .privacy()
            .reset(PrivacyService::Location, "com.apple.Maps")?;
        mock::device()?.shutdown()?;

        Ok(())
    }

    #[test]
    #[serial]
    fn test_privacy_reset_all() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?
            .privacy()
            .grant(PrivacyService::Location, "com.apple.Maps")?;
        mock::device()?
            .privacy()
            .reset_all(PrivacyService::Location)?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
