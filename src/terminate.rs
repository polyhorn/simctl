use super::{Device, Result, Validate};

impl Device {
    /// Terminates a running application with the given bundle ID on this
    /// device.
    pub fn terminate(&self, bundle_id: &str) -> Result<()> {
        self.simctl()
            .command("terminate")
            .arg(&self.udid)
            .arg(bundle_id)
            .status()?
            .validate()
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_terminate() -> Result<()> {
        mock::device()?.boot()?;

        let result = Arc::new(Mutex::new(None));

        std::thread::spawn({
            let result = result.clone();

            move || {
                fn test_terminate_inner() -> Result<()> {
                    std::thread::sleep(std::time::Duration::from_secs(1));

                    mock::device()?.terminate("com.apple.mobilesafari")
                }

                let mut result = result.lock().unwrap();
                result.replace(Some(test_terminate_inner()));
            }
        });

        mock::device()?.launch("com.apple.mobilesafari").exec()?;

        let result = result.lock().unwrap().take().flatten();
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());

        mock::device()?.shutdown()?;

        Ok(())
    }
}
