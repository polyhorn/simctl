use std::ops::Deref;

use super::list::DeviceInfo;
use super::Simctl;

/// Wrapper around a single device returned by `simctl`.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Device {
    simctl: Simctl,
    info: DeviceInfo,
}

impl Device {
    pub(crate) fn new(simctl: Simctl, info: DeviceInfo) -> Device {
        Device { simctl, info }
    }

    /// Returns an instance to the Simctl wrapper that was used to retrieve this
    /// device.
    pub fn simctl(&self) -> &Simctl {
        &self.simctl
    }

    /// Returns information about this device. This is also accessible through
    /// [`Device::deref`].
    pub fn info(&self) -> &DeviceInfo {
        &self.info
    }
}

impl Deref for Device {
    type Target = DeviceInfo;

    fn deref(&self) -> &Self::Target {
        &self.info
    }
}

/// Trait that makes it easy to filter an iterator over devices by availability
/// or name.
pub trait DeviceQuery<'a>: Iterator<Item = &'a Device> {
    /// Filters this iterator down to only available devices.
    fn available(self) -> Available<'a, Self>;

    /// Filters this iterator down to only devices with a matching name. Note
    /// that names are not necessarily unique. Trivially, they can be shared
    /// among several devices of the same type but with different runtimes (e.g.
    /// iOS 11.0 and iOS 12.0).
    fn by_name<'b>(self, name: &'b str) -> ByName<'a, 'b, Self>;
}

pub struct Available<'a, I>(I)
where
    I: Iterator<Item = &'a Device> + ?Sized;

impl<'a, I> Iterator for Available<'a, I>
where
    I: Iterator<Item = &'a Device> + ?Sized,
{
    type Item = &'a Device;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.0.next() {
            if next.is_available {
                return Some(next);
            }
        }

        None
    }
}

pub struct ByName<'a, 'b, I>(&'b str, I)
where
    I: Iterator<Item = &'a Device> + ?Sized;

impl<'a, I> Iterator for ByName<'a, '_, I>
where
    I: Iterator<Item = &'a Device> + ?Sized,
{
    type Item = &'a Device;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(next) = self.1.next() {
            if next.name == self.0 {
                return Some(next);
            }
        }

        None
    }
}

impl<'a, I> DeviceQuery<'a> for I
where
    I: Iterator<Item = &'a Device>,
{
    fn available(self) -> Available<'a, Self> {
        Available(self)
    }

    fn by_name<'b>(self, name: &'b str) -> ByName<'a, 'b, Self> {
        ByName(name, self)
    }
}
