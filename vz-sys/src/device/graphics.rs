//! graphics device module

use crate::foundation::{Id, NSArray, NSInteger};
use crate::objc::{alloc, constructor, instancetype, property, protocol};

use objc::rc::StrongPtr;

/// The base class for a graphics device configuration.
pub trait VZGraphicsDeviceConfiguration {
    fn id(&self) -> Id;
}

/// The configuration for a Mac graphics device.
pub struct VZMacGraphicsDisplayConfiguration(StrongPtr);

impl VZMacGraphicsDisplayConfiguration {
    /// Create a display configuration suitable for showing on the specified screen.
    ///
    /// # Safety
    /// The type `NSSize` must be a valid struct type of `NSSize` [^1].
    ///
    /// [^1]: https://developer.apple.com/documentation/foundation/nssize?language=objc
    pub unsafe fn new_for<NSSize>(screen: Id, size_in_points: NSSize) -> Self {
        Self(instancetype![
            alloc!(VZMacGraphicsDisplayConfiguration),
            initWithScreen: screen
            sizeInPoints: size_in_points
        ])
    }

    /// Create a display configuration with the specified pixel dimensions and pixel density.
    pub fn new_with(
        width_in_pixels: NSInteger,
        height_in_pixels: NSInteger,
        pixels_per_inch: NSInteger,
    ) -> Self {
        Self(instancetype![
            alloc!(VZMacGraphicsDisplayConfiguration),
            initWithWidthInPixels: width_in_pixels
            heightInPixels: height_in_pixels
            pixelsPerInch: pixels_per_inch
        ])
    }
}

/// Configuration for a display attached to a Mac graphics device.
pub struct VZMacGraphicsDeviceConfiguration(StrongPtr);

constructor!(
    /// Creates a new Mac graphics device configuration.
    VZMacGraphicsDeviceConfiguration
);

protocol!(
    VZGraphicsDeviceConfiguration,
    VZMacGraphicsDeviceConfiguration,
);

impl VZMacGraphicsDeviceConfiguration {
    property!(displays: NSArray<VZMacGraphicsDisplayConfiguration> { set: pub setDisplays; });
}

/// The configuration for a Virtio graphics device that configures the dimensions of the graphics
/// device for a Linux VM.
pub struct VZVirtioGraphicsScanoutConfiguration(StrongPtr);

impl VZVirtioGraphicsScanoutConfiguration {
    /// Creates a Virtio graphics device with the specified dimensions.
    pub fn new(width_in_pixels: NSInteger, height_in_pixels: NSInteger) -> Self {
        Self(instancetype![
            alloc!(VZVirtioGraphicsScanoutConfiguration),
            initWithWidthInPixels: width_in_pixels
            heightInPixels: height_in_pixels
        ])
    }

    unsafe fn id(&self) -> Id {
        *self.0
    }
}

/// Configuration that represents the configuration of a Virtio graphics device for a Linux VM.
pub struct VZVirtioGraphicsDeviceConfiguration(StrongPtr);

constructor!(
    /// Creates a new Virtio graphics device.
    VZVirtioGraphicsDeviceConfiguration
);

protocol!(
    VZGraphicsDeviceConfiguration,
    VZVirtioGraphicsDeviceConfiguration,
);

impl VZVirtioGraphicsDeviceConfiguration {
    property!(scanouts: VZVirtioGraphicsScanoutConfiguration { set: pub setScanouts; });
}
