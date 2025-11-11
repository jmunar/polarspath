pub mod sample {
    include!(concat!(env!("OUT_DIR"), "/sample.rs"));
}

#[cfg(feature = "extension-module")]
include!(concat!(env!("OUT_DIR"), "/extension_generated.rs"));
