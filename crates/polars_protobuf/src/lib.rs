// mod base;
mod traits;

// Build module is available when the "build" feature is enabled
#[cfg(feature = "build")]
pub mod build;

// pub use base::encode;
pub use traits::ArrowMessage;
