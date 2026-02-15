mod interop;
mod options;
mod reader;
mod runtime;
mod scan;

#[cfg(feature = "extension-module")]
mod python;

pub use options::LanceScanOptions;
pub use reader::read_lance;
pub use scan::scan_lance;

#[doc(hidden)]
pub mod __test_utils {
    pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
        super::runtime::block_on(future)
    }
}
