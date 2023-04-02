//! entropy device module

use crate::foundation::Id;
use crate::objc::{constructor, protocol};

use objc::rc::StrongPtr;

/// common configure of entropy device
pub trait VZEntropyDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of entropy device
pub struct VZVirtioEntropyDeviceConfiguration(StrongPtr);

constructor!(VZVirtioEntropyDeviceConfiguration);
protocol!(
    VZEntropyDeviceConfiguration,
    VZVirtioEntropyDeviceConfiguration,
);
