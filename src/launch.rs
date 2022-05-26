//! Supporting types for the `simctl launch` subcommand.

use std::fmt::Display;
use std::path::Path;
use std::process::Stdio;
use std::{ffi::OsStr, process::Child};

use super::{Device, Result, Validate};

/// Builder that can be used to customize the launch of an application.
#[derive(Debug)]
pub struct Launch<'a> {
    device: Device,
    bundle_id: &'a str,
    wait_for_debugger: bool,
    terminate_running_process: bool,
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

    /// Indicates whether the application should terminate previous launched app as well as whether
    /// it should terminate on exit.
    pub fn terminate_running_process(&mut self, terminate: bool) -> &mut Launch<'a> {
        self.terminate_running_process = terminate;
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

        command.envs(self.envs.iter().map(|(k, v)| (k, v)));

        command.arg(&self.device.udid);
        command.arg(self.bundle_id);

        command.args(&self.args);

        command.output()?.validate()
    }

    /// Spawn launch.
    ///
    /// like execute but instead return a child handler, however unlike exec,
    ///
    /// This more convinenet when you want to listen to stdout or stderr and managed the process it
    /// self.
    ///
    /// NOTE: This will ignore stdout and stderr configuration.
    /// NOTE: This will set --console to true by default unless use_pty is true.
    ///
    pub fn spawn(&mut self) -> Result<Child> {
        let mut command = self.device.simctl().command("launch");

        if self.wait_for_debugger {
            command.arg("--wait-for-debugger");
        }

        if self.terminate_running_process {
            command.arg("--terminate-running-process");
        }

        match self.use_pty {
            Some(true) => command.arg("--console-pty"),
            _ => command.arg("--console"),
        };

        command.stderr(Stdio::piped());
        command.stdout(Stdio::piped());
        command.envs(self.envs.iter().map(|(k, v)| (k, v)));
        command.arg(&self.device.udid);
        command.arg(self.bundle_id);
        command.args(&self.args);

        Ok(command.spawn()?)
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
            terminate_running_process: false,
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
