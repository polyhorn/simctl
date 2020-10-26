use std::path::Path;

use super::{Device, Result, Validate};

impl Device {
    /// Installs an .app folder from the given path onto this device. If the
    /// app (or an earlier version) already existed on this device, its app
    /// container (see [`Device::get_app_container`]) will be
    /// overwritten while the other containers remain unchanged (i.e. data
    /// persists between upgrades).
    pub fn install(&self, path: &Path) -> Result<()> {
        self.simctl()
            .command("install")
            .arg(&self.udid)
            .arg(&path)
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
    fn test_install() -> Result<()> {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        path.push("tests/Example.app");

        mock::device()?.boot()?;
        mock::device()?.install(&path)?;
        mock::device()?.uninstall("com.glacyr.simctl.Example")?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
