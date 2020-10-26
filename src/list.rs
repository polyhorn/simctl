//! Supporting types for the `simctl list` subcommand.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;

use super::{Device, Result, Simctl};

/// Indicates the state of a device.
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum DeviceState {
    /// Indicates that the device is booted.
    Booted,

    /// Indicates that the device is shutdown.
    Shutdown,

    /// Indicates that the device is in an unknown state.
    #[serde(other)]
    Unknown,
}

/// Indicates the state of a pair of devices.
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq)]
pub enum DevicePairState {
    /// Indicates that this pair is unavailable because one of its components is
    /// unavailable.
    #[serde(rename = "(unavailable)")]
    Unavailable,

    /// Indicates that this pair is active but not connected.
    #[serde(rename = "(active, disconnected)")]
    ActiveDisconnected,

    /// Indicates that this pair is in a state that is not (yet) recognized by
    /// this library.
    #[serde(other)]
    Unknown,
}

/// Information about a device type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeviceType {
    /// Contains the minimum runtime version that this device type supports.
    /// This is relevant for devices that are newer than the oldest runtime that
    /// has been registered with `simctl`.
    #[serde(rename = "minRuntimeVersion")]
    pub min_runtime_version: usize,

    /// Contains the maximum runtime version that this device type supports.
    /// This is relevant for devices that have been deprecated before the newest
    /// runtime that has been registered with `simctl`.
    #[serde(rename = "maxRuntimeVersion")]
    pub max_runtime_version: usize,

    /// Contains a path to the bundle of this device type. This is usually not
    /// relevant to end-users.
    #[serde(rename = "bundlePath")]
    pub bundle_path: PathBuf,

    /// Contains a human-readable name for this device type.
    pub name: String,

    /// Contains a unique identifier for this device type.
    pub identifier: String,

    /// Contains a machine-readable name for the product family of this device
    /// type.
    #[serde(rename = "productFamily")]
    pub product_family: String,
}

/// Information about a runtime.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Runtime {
    /// Contains a path to the bundle of this runtime. This is usually not
    /// relevant to end-users.
    #[serde(rename = "bundlePath")]
    pub bundle_path: PathBuf,

    /// Contains the build version of this runtime. This is usually not relevant
    /// to end-users.
    #[serde(rename = "buildversion")]
    pub build_version: String,

    /// Contains the root of this runtime. This is usually not relevant to
    /// end-users.
    #[serde(rename = "runtimeRoot")]
    pub runtime_root: PathBuf,

    /// Contains a unique identifier for this runtime.
    pub identifier: String,

    /// Contains a human-readable version string for this runtime.
    pub version: String,

    /// Indicates if this runtime is available. This is false when the runtime
    /// was first created (automatically) with an older version of Xcode that
    /// shipped with an older version of the iOS simulator and after upgrading
    /// to a newer version. In that case, Xcode no longer has the runtime bundle
    /// for this older runtime, but it will still be registered by `simctl`.
    /// However, it's not possible to boot a device with an unavailable runtime.
    #[serde(rename = "isAvailable")]
    pub is_available: bool,

    /// Contains a human-readable name for this runtime.
    pub name: String,
}

/// Information about a device.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeviceInfo {
    /// Note: this field is not directly present in JSON. Instead, the JSON
    /// representation is a hashmap of runtime IDs (keys) and devices (values)
    /// that we later connect during deserialization.
    #[serde(skip_deserializing)]
    pub runtime_identifier: String,

    /// If this device is not available (see [`DeviceInfo::is_available`]), this
    /// will contain a (slightly) more detailed explanation for its
    /// unavailability.
    #[serde(default, rename = "availabilityError")]
    pub availability_error: Option<String>,

    /// Contains the path where application data is stored.
    #[serde(rename = "dataPath")]
    pub data_path: PathBuf,

    /// Contains the path where logs are written to.
    #[serde(rename = "logPath")]
    pub log_path: PathBuf,

    /// Contains a unique identifier for this device.
    pub udid: String,

    /// Indicates if this device is available. Also see
    /// [`Runtime::is_available`].
    #[serde(rename = "isAvailable")]
    pub is_available: bool,

    /// This corresponds to [`DeviceType::identifier`]. This is missing for
    /// devices whose device type has since been removed from Xcode.
    #[serde(default, rename = "deviceTypeIdentifier")]
    pub device_type_identifier: String,

    /// Contains the state of this device.
    pub state: DeviceState,

    /// Contains the name of this device.
    pub name: String,
}

/// Short summary of a device that is used as part of a device pair.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeviceSummary {
    /// Contains the name of this device.
    pub name: String,

    /// Contains a unique identifier for this device.
    pub udid: String,

    /// Contains the state of this device.
    pub state: DeviceState,
}

/// Information about a device pair.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DevicePair {
    /// Note: this field is not directly present in JSON. Instead, the JSON
    /// representation is a hashmap of runtime IDs (keys) and devices (values)
    /// that we later connect during deserialization.
    #[serde(skip_deserializing)]
    pub udid: String,

    /// Contains a summary of the watch device.
    pub watch: DeviceSummary,

    /// Contains a summary of the phone device.
    pub phone: DeviceSummary,

    /// Contains the state of this device pair.
    pub state: DevicePairState,
}

/// Wrapper around the `simctl list` subcommand's output.
#[derive(Debug)]
pub struct List {
    simctl: Simctl,
    device_types: Vec<DeviceType>,
    runtimes: Vec<Runtime>,
    devices: Vec<Device>,
    pairs: Vec<DevicePair>,
}

impl List {
    /// Refreshes the `simctl list` subcommand's output.
    pub fn refresh(&mut self) -> Result<()> {
        let mut command = self.simctl.command("list");
        command.arg("-j");
        command.stdout(Stdio::piped());
        let output = command.output()?;
        let output: ListOutput = serde_json::from_slice(&output.stdout)?;
        self.device_types = output.device_types;
        self.runtimes = output.runtimes;
        self.devices = output
            .devices
            .into_iter()
            .map(|(runtime, devices)| {
                let simctl = self.simctl.clone();

                devices.into_iter().map(move |device| {
                    Device::new(
                        simctl.clone(),
                        DeviceInfo {
                            runtime_identifier: runtime.clone(),
                            ..device
                        },
                    )
                })
            })
            .flatten()
            .collect();
        self.pairs = output
            .pairs
            .into_iter()
            .map(move |(udid, pair)| DevicePair { udid, ..pair })
            .collect();
        Ok(())
    }

    /// Returns all device types that have been registered with `simctl`.
    pub fn device_types(&self) -> &[DeviceType] {
        &self.device_types
    }

    /// Returns all runtimes that have been registered with `simctl`.
    pub fn runtimes(&self) -> &[Runtime] {
        &self.runtimes
    }

    /// Returns all devices that have been registered with `simctl`.
    pub fn devices(&self) -> &[Device] {
        &self.devices
    }

    /// Returns all device pairs that have been registered with `simctl`.
    pub fn pairs(&self) -> &[DevicePair] {
        &self.pairs
    }
}

#[derive(Debug, Default, Deserialize)]
struct ListOutput {
    #[serde(rename = "devicetypes")]
    device_types: Vec<DeviceType>,
    runtimes: Vec<Runtime>,
    devices: HashMap<String, Vec<DeviceInfo>>,
    pairs: HashMap<String, DevicePair>,
}

impl Simctl {
    /// Returns a list of all device types, runtimes, devices and device pairs
    /// that have been registered with `simctl`.
    pub fn list(&self) -> Result<List> {
        let mut list = List {
            simctl: self.clone(),
            device_types: vec![],
            devices: vec![],
            pairs: vec![],
            runtimes: vec![],
        };
        list.refresh()?;
        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() -> Result<()> {
        let simctl = Simctl::new();
        let _ = simctl.list()?;
        Ok(())
    }
}
