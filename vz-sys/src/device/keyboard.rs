//! keyboard module

use crate::foundation::Id;
use crate::objc::{constructor, protocol};

use objc::rc::StrongPtr;

/// The base class for a configuring a keyboard.
pub trait VZKeyboardConfiguration {
    fn id(&self) -> Id;
}

/// A device that defines the configuration for a USB keyboard.
pub struct VZUSBKeyboardConfiguration(StrongPtr);

constructor!(VZUSBKeyboardConfiguration);
protocol!(VZKeyboardConfiguration, VZUSBKeyboardConfiguration);
