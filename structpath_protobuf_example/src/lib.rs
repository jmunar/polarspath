pub mod structpath_protobuf_example {
    include!(concat!(env!("OUT_DIR"), "/structpath_protobuf_example.rs"));
}

#[cfg(feature = "extension-module")]
include!(concat!(env!("OUT_DIR"), "/extension_generated.rs"));
