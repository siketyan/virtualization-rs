//! memory device module

use crate::foundation::Id;
use crate::objc::{constructor, protocol};

use objc::rc::StrongPtr;

/// common configure of memory balloon device
pub trait VZMemoryBalloonDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of memory balloon device through the Virtio interface
pub struct VZVirtioTraditionalMemoryBalloonDeviceConfiguration(StrongPtr);

constructor!(VZVirtioTraditionalMemoryBalloonDeviceConfiguration);
protocol!(
    VZMemoryBalloonDeviceConfiguration,
    VZVirtioTraditionalMemoryBalloonDeviceConfiguration,
);
