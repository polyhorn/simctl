//! Supporting types for the `simctl status_bar` subcommand.

use super::{Device, Result, Validate};

/// Controls the battery state that is shown in the status bar.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BatteryState {
    /// Indicates that the battery is charging.
    Charging,

    /// Indicates that the battery is fully charged.
    Charged,

    /// Indicates that the battery is discharging (i.e. disconnected from an
    /// external power source).
    Discharging,
}

/// Controls the cellular mode that is shown in the status bar.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CellularMode {
    /// Indicates that this device does not support cellular connectivity.
    NotSupported,

    /// Indicates that the device is currently searching for a cellular network.
    Searching,

    /// Indicates that the device has failed to find a cellular network.
    Failed,

    /// Indicates that the device is currently connected to a cellular network.
    Active,
}

/// Controls the data network that is shown in the status bar.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataNetworkType {
    /// Indicates that the device is connected to a Wi-Fi network.
    Wifi,

    /// Indicates that the device is connected to a 3G cellular network.
    Cell3G,

    /// Indicates that the device is connected to a 4G cellular network.
    Cell4G,

    /// Indicates that the device is connected to a LTE cellular network.
    CellLte,

    /// Indicates that the device is connected to a LTE-Advanced cellular
    /// network.
    CellLteA,

    /// Indicates that the device is connected to a LTE+ cellular network.
    CellLtePlus,
}

/// Controls the Wi-Fi mode that is shown in the status bar.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WifiMode {
    /// Indicates that the device is searching for a Wi-Fi network.
    Searching,

    /// Indicates that the device failed to find a Wi-Fi network.
    Failed,

    /// Indicates that the device is currently connected to a Wi-Fi network.
    Active,
}

/// Wrapper around the `simctl status_bar` subcommand.
pub struct StatusBar {
    device: Device,
}

impl StatusBar {
    /// Clears any previous override.
    pub fn clear(&self) -> Result<()> {
        self.device
            .simctl()
            .command("status_bar")
            .arg(&self.device.udid)
            .arg("clear")
            .status()?
            .validate()
    }

    /// Creates a new empty override that can be applied to this status bar.
    pub fn empty_override(&self) -> StatusBarOverride {
        StatusBarOverride {
            device: self.device.clone(),
            time: None,
            data_network: None,
            wifi_mode: None,
            wifi_bars: None,
            cellular_mode: None,
            cellular_bars: None,
            operator_name: None,
            battery_state: None,
            battery_level: None,
        }
    }
}

/// Builder that can be used to customize the status bar override before
/// applying it.
pub struct StatusBarOverride {
    device: Device,
    time: Option<String>,
    data_network: Option<DataNetworkType>,
    wifi_mode: Option<WifiMode>,
    wifi_bars: Option<usize>,
    cellular_mode: Option<CellularMode>,
    cellular_bars: Option<usize>,
    operator_name: Option<String>,
    battery_state: Option<BatteryState>,
    battery_level: Option<usize>,
}

impl StatusBarOverride {
    /// Updates the time that is shown in the status bar.
    pub fn time(&mut self, time: &str) -> &mut StatusBarOverride {
        self.time = Some(time.to_owned());
        self
    }

    /// Updates the data network type that is shown in the status bar (e.g. 3G
    /// or 4G).
    pub fn data_network(&mut self, data_network: DataNetworkType) -> &mut StatusBarOverride {
        self.data_network = Some(data_network);
        self
    }

    /// Updates the wifi mode that is shown in the status bar (i.e. whether it's
    /// active or not).
    pub fn wifi_mode(&mut self, wifi_mode: WifiMode) -> &mut StatusBarOverride {
        self.wifi_mode = Some(wifi_mode);
        self
    }

    /// Updates the number of wifi bars that are shown in the status bar. This
    /// is only applicable if the wifi mode is [`WifiMode::Active`].
    pub fn wifi_bars(&mut self, wifi_bars: usize) -> &mut StatusBarOverride {
        self.wifi_bars = Some(wifi_bars);
        self
    }

    /// Updates the cellular mode that is shown in the status bar (i.e. whether
    /// it's active or not).
    pub fn cellular_mode(&mut self, cellular_mode: CellularMode) -> &mut StatusBarOverride {
        self.cellular_mode = Some(cellular_mode);
        self
    }

    /// Updates the number of cellular bars that are shown in the status bar.
    /// This is only applicable if the cellular mode is
    /// [`CellularMode::Active`].
    pub fn cellular_bars(&mut self, cellular_bars: usize) -> &mut StatusBarOverride {
        self.cellular_bars = Some(cellular_bars);
        self
    }

    /// Updates the operator name that is shown in the status bar. This is only
    /// applicable if the cellular mode is [`CellularMode::Active`].
    pub fn operator_name(&mut self, name: &str) -> &mut StatusBarOverride {
        self.operator_name = Some(name.to_owned());
        self
    }

    /// Updates the battery state that is shown in the status bar.
    pub fn battery_state(&mut self, state: BatteryState) -> &mut StatusBarOverride {
        self.battery_state = Some(state);
        self
    }

    /// Updates the battery state that is shown in the status bar. This is only
    /// applicable if the battery state is [`BatteryState::Discharging`].
    pub fn battery_level(&mut self, level: usize) -> &mut StatusBarOverride {
        self.battery_level = Some(level);
        self
    }

    /// Applies this override to the status bar.
    pub fn apply(&self) -> Result<()> {
        let mut command = self.device.simctl().command("status_bar");

        command.arg(&self.device.udid).arg("override");

        if let Some(time) = self.time.as_ref() {
            command.arg("--time").arg(time);
        }

        if let Some(network) = self.data_network.as_ref() {
            command.arg("--dataNetwork").arg(match network {
                DataNetworkType::Wifi => "wifi",
                DataNetworkType::Cell3G => "3g",
                DataNetworkType::Cell4G => "4g",
                DataNetworkType::CellLte => "lte",
                DataNetworkType::CellLteA => "lte-a",
                DataNetworkType::CellLtePlus => "lte+",
            });
        }

        if let Some(mode) = self.wifi_mode.as_ref() {
            command.arg("--wifiMode").arg(match mode {
                WifiMode::Searching => "searching",
                WifiMode::Failed => "failed",
                WifiMode::Active => "active",
            });
        }

        if let Some(bars) = self.wifi_bars.as_ref() {
            command.arg("--wifiBars").arg(bars.to_string());
        }

        if let Some(mode) = self.cellular_mode.as_ref() {
            command.arg("--cellularMode").arg(match mode {
                CellularMode::NotSupported => "notSupported",
                CellularMode::Searching => "searching",
                CellularMode::Failed => "failed",
                CellularMode::Active => "active",
            });
        }

        if let Some(bars) = self.cellular_bars.as_ref() {
            command.arg("--cellularBars").arg(bars.to_string());
        }

        if let Some(name) = self.operator_name.as_ref() {
            command.arg("--operatorName").arg(&name);
        }

        if let Some(state) = self.battery_state.as_ref() {
            command.arg("--batteryState").arg(match state {
                BatteryState::Charging => "charging",
                BatteryState::Charged => "charged",
                BatteryState::Discharging => "discharging",
            });
        }

        if let Some(level) = self.battery_level.as_ref() {
            command.arg("--batteryLevel").arg(level.to_string());
        }

        command.status()?.validate()
    }
}

impl Device {
    /// Returns a wrapper around the `simctl status_bar` subcommand.
    pub fn status_bar(&self) -> StatusBar {
        StatusBar {
            device: self.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use super::*;
    use crate::mock;

    #[test]
    #[serial]
    fn test_status_bar() -> Result<()> {
        mock::device()?.boot()?;
        mock::device()?
            .status_bar()
            .empty_override()
            .time("00:00")
            .data_network(DataNetworkType::Cell4G)
            .cellular_mode(CellularMode::Active)
            .cellular_bars(3)
            .operator_name("Babel")
            .battery_state(BatteryState::Discharging)
            .battery_level(42)
            .apply()?;
        mock::device()?.status_bar().clear()?;
        mock::device()?.shutdown()?;

        Ok(())
    }
}
