use std::process::{ExitStatus, Output};

/// Error that is returned when the CLI does not successfully complete a
/// request, or when the library encountered a problem while generating the
/// request or while interpreting its response.
#[derive(Debug)]
pub enum Error {
    /// This error is returned when the CLI exits with a non-zero exit code.
    Output {
        /// Contains the output written to stdout before the CLI exited with a
        /// non-zero exit code.
        stdout: String,

        /// Contains the output written to stderr before the CLI exited with a
        /// non-zero exit code.
        stderr: String,

        /// Contains the exit status.
        status: ExitStatus,
    },

    /// This error is returned when the library failed spawning a new process
    /// that runs the CLI. Most likely, this is caused by an incorrect Xcode
    /// path. If the Xcode path was set automatically, Xcode is probably not
    /// installed. If the Xcode path was set manually, it's probably incorrect.
    /// Make sure that it ends with `Xcode(-*).app` (where * can be an optional
    /// suffix to distinguish between stable and beta).
    Io(std::io::Error),

    /// This error is returned when the library failed to deserialize the
    /// response of `simctl list -j` (in [`crate::list`]) or when it failed to
    /// serialize a request for `simctl push` (in [`crate::push`]).
    Json(serde_json::Error),

    /// This error is returned when the library failed to interpret the CLI's
    /// response as a UTF-8 encoded string.
    Utf8(std::string::FromUtf8Error),
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Json(error)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Utf8(error)
    }
}

/// Partial application of the standard `Result` type, with the simctl [`Error`]
/// pre-applied.
pub type Result<T> = std::result::Result<T, Error>;

pub trait Validate {
    fn validate(self) -> Result<()>;
    fn validate_with_output(self) -> Result<Output>;
}

impl Validate for Output {
    fn validate(self) -> Result<()> {
        let _ = self.validate_with_output()?;
        Ok(())
    }

    fn validate_with_output(self) -> Result<Output> {
        match self.status.success() {
            true => Ok(self),
            false => Err(Error::Output {
                stdout: String::from_utf8(self.stdout).unwrap(),
                stderr: String::from_utf8(self.stderr).unwrap(),
                status: self.status,
            }),
        }
    }
}
