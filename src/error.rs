use std::process::ExitStatus;

/// Error that is returned when the CLI does not successfully complete a
/// request, or when the library encountered a problem while generating the
/// request or while interpreting its response.
#[derive(Debug)]
pub enum Error {
    /// This error is returned when the CLI exits with a non-zero exit code.
    ExitStatus(ExitStatus),

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
}

impl Validate for ExitStatus {
    fn validate(self) -> Result<()> {
        match self.success() {
            true => Ok(()),
            false => Err(Error::ExitStatus(self)),
        }
    }
}
