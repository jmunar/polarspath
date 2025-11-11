mod base;

// Build module is available when the "build" feature is enabled
#[cfg(feature = "build")]
pub mod build;
#[cfg(feature = "build")]
mod string;

pub use base::{get_type, get_value};
