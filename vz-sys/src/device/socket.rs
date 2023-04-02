//! socket device module

use crate::foundation::Id;

/// common configure of socket device
pub trait VZSocketDeviceConfiguration {
    fn id(&self) -> Id;
}
