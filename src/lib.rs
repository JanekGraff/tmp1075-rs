//! # TMP1075
//! A platform agnostic driver to interface with the TMP1075 temperature sensor.
//! The driver supports async and blocking mode, selectable wit hthe `async` and `blocking` features.
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(warnings)]
#![forbid(unsafe_code)]
#![cfg_attr(not(test), no_std)]

#[cfg(all(feature = "blocking", feature = "async"))]
compile_error!("Feature \"blocking\" and feature \"async\" cannot be enabled at the same time");
#[cfg(not(any(feature = "blocking", feature = "async")))]
compile_error!("Either feature \"blocking\" or feature \"async\" must be anbled");

mod register_settings;
mod registers;
mod tmp1075;

pub use register_settings::*;

#[cfg(feature = "blocking")]
pub use tmp1075::Tmp1075;

#[cfg(feature = "async")]
pub use tmp1075::Tmp1075Async as Tmp1075;
