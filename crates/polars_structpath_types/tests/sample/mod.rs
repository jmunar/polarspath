mod base;
mod enumpath;
mod structpath;
mod test;

pub use base::{SampleEnum, SampleStruct, SampleSubstruct};
#[allow(unused)]
pub use test::{sample_struct, sample_struct_null};
