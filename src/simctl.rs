use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{Result, Validate};

/// Wrapper around the `simctl` utility.
#[derive(Clone, Debug)]
pub struct Simctl {
    xcode_path: PathBuf,
}

impl Simctl {
    /// Returns a new instance of the Rust wrapper around the `simctl` utility.
    pub fn new() -> Simctl {
        Simctl::with_xcode(Path::new("/Applications/Xcode.app"))
    }

    /// Returns a new wrapper around the `simctl` utility with the given Xcode
    /// path. Use this function if Xcode is not installed in the default path or
    /// if you want to distinguish between multiple installations of Xcode (e.g.
    /// stable and beta).
    pub fn with_xcode(path: &Path) -> Simctl {
        Simctl {
            xcode_path: path.to_path_buf(),
        }
    }

    /// Returns a new command that will invoke the `simctl` binary with the
    /// given subcommand.
    pub fn command(&self, name: &str) -> Command {
        let mut command = Command::new(self.xcode_path.join("Contents/Developer/usr/bin/simctl"));
        command.arg(name);
        command.stdout(Stdio::null());
        command.stderr(Stdio::null());
        command
    }

    /// Opens the Simulator.app that corresponds to this instance of `simctl`
    /// (in case of multiple Xcode installations).
    pub fn open(&self) -> Result<()> {
        Command::new("open")
            .arg(self.xcode_path.join("Contents/Developer/Simulator.app"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?
            .validate()
    }
}
