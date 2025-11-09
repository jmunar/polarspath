mod base;
#[cfg(feature = "extension-module")]
mod extension;

pub use base::{get_type, get_value};
