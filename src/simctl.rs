use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{Result, Validate};

/// Wrapper around the `simctl` utility.
#[derive(Clone, Debug)]
pub struct Simctl {
    developer_dir: PathBuf,
}

impl Simctl {
    /// Returns a new instance of the Rust wrapper around the `simctl` utility.
    pub fn new() -> Simctl {
        if let Some(developer_dir) = std::env::var_os("DEVELOPER_DIR") {
            Simctl::with_developer_dir(&Path::new(&developer_dir))
        } else {
            let output = Command::new("xcode-select")
                .arg("--print-path")
                .stdout(Stdio::piped())
                .output()
                .unwrap();

            let output = String::from_utf8(output.stdout).unwrap();
            let path = Path::new(output.trim());

            Simctl::with_developer_dir(path)
        }
    }

    /// Returns a new wrapper around the `simctl` utility with the given
    /// developer dir. Use this function if Xcode is not installed in the
    /// default path or if you want to distinguish between multiple
    /// installations of Xcode (e.g. stable and beta).
    pub fn with_developer_dir(path: &Path) -> Simctl {
        Simctl {
            developer_dir: path.to_path_buf(),
        }
    }

    /// Returns a new wrapper around the `simctl` utility with the given Xcode
    /// path. Use this function if Xcode is not installed in the default path or
    /// if you want to distinguish between multiple installations of Xcode (e.g.
    /// stable and beta).
    pub fn with_xcode(path: &Path) -> Simctl {
        Simctl::with_xcode(&path.join("Contents/Developer"))
    }

    /// Returns a new command that will invoke the `simctl` binary with the
    /// given subcommand.
    pub fn command(&self, name: &str) -> Command {
        let mut command = Command::new(self.developer_dir.join("usr/bin/simctl"));
        command.arg(name);
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        command
    }

    /// Opens the Simulator.app that corresponds to this instance of `simctl`
    /// (in case of multiple Xcode installations).
    pub fn open(&self) -> Result<()> {
        Command::new("open")
            .arg(
                self.developer_dir
                    .join("Applications")
                    .join("Simulator.app"),
            )
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()?
            .validate()
    }
}
