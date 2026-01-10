mod base;
mod traits;

#[cfg(feature = "build")]
pub mod build;

pub use base::{decode, encode};
pub use traits::ArrowMessage;
