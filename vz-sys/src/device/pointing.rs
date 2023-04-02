//! pointing device module

use crate::foundation::Id;

use crate::objc::{constructor, protocol};
use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// The base class for a pointing device configuration.
pub trait VZPointingDeviceConfiguration {
    fn id(&self) -> Id;
}

/// The class that represents the configuration for a Mac trackpad.
///
/// # Note
/// The framework recognizes this device in virtual machines running macOS 13 and later. To support
/// both macOS 13.0 and earlier guests, set pointingDevices to an array that contains both a
/// [`VZMacTrackpadConfiguration`] and a [`VZUSBScreenCoordinatePointingDeviceConfiguration`] object.
pub struct VZMacTrackpadConfiguration(StrongPtr);

constructor!(
    /// Creates a new Mac trackpad configuration.
    VZMacTrackpadConfiguration
);

protocol!(VZPointingDeviceConfiguration, VZMacTrackpadConfiguration);

/// An object that defines the configuration for a USB pointing device that reports absolute coordinates.
pub struct VZUSBScreenCoordinatePointingDeviceConfiguration(StrongPtr);

constructor!(
    /// Creates a new pointing device.
    VZUSBScreenCoordinatePointingDeviceConfiguration
);

protocol!(
    VZPointingDeviceConfiguration,
    VZUSBScreenCoordinatePointingDeviceConfiguration,
);
