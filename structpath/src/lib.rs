#[cfg(feature = "derive")]
extern crate structpath_derive;

#[cfg(feature = "derive")]
pub use structpath_derive::{EnumPath, StructPath};

extern crate structpath_types;
pub use structpath_types::{
    indexmap, DataTypeOpt, DataTypeOptError, EnumPath, HasDataTypeOpt, IntoAnyValueWith, Path,
    PathComponent, StructPath,
};
