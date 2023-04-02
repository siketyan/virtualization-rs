//! network device module

use crate::foundation::{Id, NSString};
use crate::objc::{alloc, constructor, instancetype, new, property, protocol, retain};

use objc::rc::StrongPtr;
use objc::{class, msg_send, sel, sel_impl};

/// common behaviors for network device attachment
pub trait VZNetworkDeviceAttachment {
    fn id(&self) -> Id;
}

/// configure of NAT network device attachment
pub struct VZNATNetworkDeviceAttachment(StrongPtr);

constructor!(VZNATNetworkDeviceAttachment);
protocol!(VZNetworkDeviceAttachment, VZNATNetworkDeviceAttachment);

/// common behaviors for bridge network interface
pub trait VZBridgedNetworkInterface {
    fn id(&self) -> Id;

    property!(identifier: NSString { get: identifier; });
    property!(localized_display_name: NSString { get: localized_display_name; });
}

/// configure of bridge network device attachment
pub struct VZBridgedNetworkDeviceAttachment(StrongPtr);

impl VZBridgedNetworkDeviceAttachment {
    pub fn new<T: VZBridgedNetworkInterface>(interface: T) -> VZBridgedNetworkDeviceAttachment {
        Self(instancetype![
            alloc!(VZBridgedNetworkDeviceAttachment),
            initWithInterface: interface.id()
        ])
    }
}

protocol!(VZNetworkDeviceAttachment, VZBridgedNetworkDeviceAttachment);

/// MAC address
pub struct VZMACAddress(pub StrongPtr);

impl VZMACAddress {
    pub fn new() -> Self {
        Self(new!(VZMacAddress))
    }

    pub fn random_locally_administered_address() -> Self {
        Self(retain![
            class!(VZMACAddress),
            randomLocallyAdministeredAddress
        ])
    }

    pub fn init_with_string(s: &str) -> VZMACAddress {
        Self(instancetype![
            alloc!(VZMacAddress),
            initWithString: *NSString::new(s).0
        ])
    }

    unsafe fn id(&self) -> Id {
        *self.0
    }
}

/// common configure of network device
pub trait VZNetworkDeviceConfiguration {
    fn id(&self) -> Id;
}

/// configure of network device through the Virtio interface
pub struct VZVirtioNetworkDeviceConfiguration(StrongPtr);

constructor!(VZVirtioNetworkDeviceConfiguration);
protocol!(
    VZNetworkDeviceConfiguration,
    VZVirtioNetworkDeviceConfiguration
);

impl VZVirtioNetworkDeviceConfiguration {
    property!(attachment: T { set: pub setAttachment <T: VZNetworkDeviceAttachment>; });
    property!(mac_address: VZMACAddress { set: pub setMACAddress; });
}
