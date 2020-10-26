//! Supporting types for the `simctl get_app_container` subcommand.

use std::path::{Path, PathBuf};
use std::process::Stdio;

use super::{Device, Result, Validate};

/// Identifies a container that iOS stores a particular kind of data in.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Container {
    /// This is the .app bundle itself. Apps cannot write to their app
    /// container.
    App,

    /// This is a directory that an app can write to and read from.
    Data,

    /// This is a directory that an app can share with several other apps made
    /// by the same developer. Each app in a group can write to and read from
    /// this container.
    Group(String),
}

impl Device {
    /// Returns a path to the given container of an application with the given
    /// bundle id.
    pub fn get_app_container(&self, bundle_id: &str, container: &Container) -> Result<PathBuf> {
        let container = match container {
            Container::App => "app",
            Container::Data => "data",
            Container::Group(group) => group,
        };

        let output = self
            .simctl()
            .command("get_app_container")
            .arg(&self.udid)
            .arg(bundle_id)
            .arg(container)
            .stdout(Stdio::piped())
            .output()?;

        let output = output.validate_with_output()?;

        Ok(Path::new(String::from_utf8(output.stdout)?.trim()).to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_get_app_container() -> Result<()> {
        mock::device()?.boot()?;

        assert_eq!(mock::device()?.get_app_container("com.apple.mobilesafari", &Container::App)?.to_str().unwrap(), "/Applications/Xcode.app/Contents/Developer/Platforms/iPhoneOS.platform/Library/Developer/CoreSimulator/Profiles/Runtimes/iOS.simruntime/Contents/Resources/RuntimeRoot/Applications/MobileSafari.app");
        let _ = mock::device()?.get_app_container("com.apple.mobilesafari", &Container::Data);

        mock::device()?.shutdown()?;

        Ok(())
    }
}
