pub mod example_protobuf {
    include!(concat!(env!("OUT_DIR"), "/example_protobuf.rs"));
}

#[cfg(feature = "extension-module")]
include!(concat!(env!("OUT_DIR"), "/extension_generated.rs"));
