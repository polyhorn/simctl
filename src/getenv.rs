use std::process::Stdio;

use super::{Device, Result, Validate};

impl Device {
    /// Returns a local environment variable with the given name. Do not prepend
    /// `SIMCTL_CHILD_` to the variable name. If no variable with the given name
    /// exists, this function will return an empty string (and no error).
    pub fn getenv(&self, name: &str) -> Result<String> {
        let output = self
            .simctl()
            .command("getenv")
            .arg(&self.udid)
            .arg(&name)
            .stdout(Stdio::piped())
            .output()?;

        output.status.validate()?;

        Ok(String::from_utf8(output.stdout)?.trim().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_getenv() -> Result<()> {
        mock::device()?.boot_with_env(vec![("TEST_VAR", "Hello World!")])?;
        assert_eq!(mock::device()?.getenv("TEST_VAR")?, "Hello World!");
        assert_eq!(mock::device()?.getenv("TEST_VAR_")?, "");
        mock::device()?.shutdown()?;

        Ok(())
    }
}
