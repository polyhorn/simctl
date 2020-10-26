use super::{Device, Result, Validate};

impl Device {
    /// Uninstalls an app with the given bundle ID from this device.
    pub fn uninstall(&self, bundle_id: &str) -> Result<()> {
        self.simctl()
            .command("uninstall")
            .arg(&self.udid)
            .arg(&bundle_id)
            .output()?
            .validate()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::path::Path;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_uninstall() -> Result<()> {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf();
        path.push("tests/Example.app");

        mock::device()?.boot()?;
        mock::device()?.install(&path)?;
        mock::device()?.uninstall("com.glacyr.simctl.Example")?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
