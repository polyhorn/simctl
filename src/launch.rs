//! Supporting types for the `simctl launch` subcommand.

use std::ffi::OsStr;
use std::fmt::Display;
use std::path::Path;
use std::process::Stdio;

use super::{Device, Result, Validate};

/// Builder that can be used to customize the launch of an application.
#[derive(Debug)]
pub struct Launch<'a> {
    device: Device,
    bundle_id: &'a str,
    wait_for_debugger: bool,
    use_pty: Option<bool>,
    stdout: Option<&'a Path>,
    stderr: Option<&'a Path>,
    args: Vec<&'a OsStr>,
    envs: Vec<(String, &'a OsStr)>,
}

impl<'a> Launch<'a> {
    /// Indicates whether the application should wait for a debugger to attach.
    pub fn wait_for_debugger(&mut self, wait: bool) -> &mut Launch<'a> {
        self.wait_for_debugger = wait;
        self
    }

    /// Indicates whether the output should be written to a console with PTY.
    pub fn use_pty(&mut self, use_pty: bool) -> &mut Launch<'a> {
        self.use_pty = Some(use_pty);
        self.stdout = None;
        self.stderr = None;
        self
    }

    /// Writes stdout to the given path.
    pub fn stdout<P>(&mut self, path: &'a P) -> &mut Launch<'a>
    where
        P: AsRef<Path>,
    {
        self.use_pty = None;
        self.stdout = Some(path.as_ref());
        self
    }

    /// Writes stderr to the given path.
    pub fn stderr<P>(&mut self, path: &'a P) -> &mut Launch<'a>
    where
        P: AsRef<Path>,
    {
        self.use_pty = None;
        self.stderr = Some(path.as_ref());
        self
    }

    /// Adds an argument that will be passed to the program.
    pub fn arg<S>(&mut self, arg: &'a S) -> &mut Launch<'a>
    where
        S: AsRef<OsStr>,
    {
        self.args.push(arg.as_ref());
        self
    }

    /// Adds an environment variable that will be made available to the program.
    pub fn env<K, V>(&mut self, key: K, value: &'a V) -> &mut Launch<'a>
    where
        K: Display,
        V: AsRef<OsStr>,
    {
        self.envs
            .push((format!("SIMCTL_CHILD_{}", key), value.as_ref()));
        self
    }

    /// Executes the launch.
    pub fn exec(&mut self) -> Result<()> {
        let mut command = self.device.simctl().command("launch");

        if self.wait_for_debugger {
            command.arg("--wait-for-debugger");
        }

        if let Some(use_pty) = self.use_pty {
            match use_pty {
                true => command.arg("--console-pty"),
                false => command.arg("--console"),
            };
        }

        if let Some(stdout) = self.stdout {
            command.arg(format!("--stdout={}", stdout.display()));
        } else {
            command.stdout(Stdio::inherit());
        }

        if let Some(stderr) = self.stderr {
            command.arg(format!("--stderr={}", stderr.display()));
        } else {
            command.stderr(Stdio::inherit());
        }

        command.arg(&self.device.udid);
        command.arg(self.bundle_id);

        command.args(&self.args);

        command.output()?.validate()
    }
}

impl Device {
    /// Returns a builder that can be used to customize the launch of an app
    /// with the given bundle ID on this device.
    pub fn launch<'a>(&self, bundle_id: &'a str) -> Launch<'a> {
        Launch {
            device: self.clone(),
            bundle_id,
            wait_for_debugger: false,
            use_pty: Some(false),
            stdout: None,
            stderr: None,
            args: vec![],
            envs: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mock;

    use serial_test::serial;

    #[test]
    #[serial]
    fn test_launch() -> Result<()> {
        mock::device()?.boot()?;

        let path = "/dev/zero";

        mock::device()?
            .launch("com.apple.mobilesafari")
            .stdout(&path)
            .stderr(&path)
            .exec()?;

        mock::device()?.shutdown()?;

        Ok(())
    }
}
