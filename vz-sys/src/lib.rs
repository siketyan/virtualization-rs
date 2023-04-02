#![allow(improper_ctypes)]
#![allow(clippy::new_without_default)]

//! virtualization-rs provides the API of the Apple [Virtualization.framework](https://developer.apple.com/documentation/virtualization?language=objc) in Rust language.
//!
//! # Examples
//! See the [simplevm](https://github.com/suzusuzu/virtualization-rs/blob/main/examples/simplevm.rs) for more details.
//!
//! The example is inspired from [SimpleVM](https://github.com/KhaosT/SimpleVM).

mod objc;

pub mod boot_loader;
pub mod device;
pub mod foundation;
pub mod vm;

pub use concat_idents::concat_idents;
