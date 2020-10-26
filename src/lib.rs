//! Rust-wrapper around the `simctl` utility that is shipped with Xcode and that
//! can be used to install apps onto one of the iOS simulator and subsequently
//! launch them.

#![warn(missing_docs)]

mod device;
mod simctl;

mod boot;
mod error;
pub mod get_app_container;
mod getenv;
mod install;
pub mod io;
pub mod keychain;
pub mod launch;
pub mod list;
mod open_url;
pub mod privacy;
pub mod push;
mod shutdown;
pub mod status_bar;
mod terminate;
pub mod ui;
mod uninstall;

#[cfg(test)]
pub mod mock;

pub use crate::simctl::Simctl;
pub use device::{Device, DeviceQuery};
pub(crate) use error::Validate;
pub use error::{Error, Result};
