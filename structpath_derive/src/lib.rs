mod enumpath;
mod string;
mod structpath;
mod utils;

use enumpath::derive_enum_path_impl;
use proc_macro::TokenStream;
use structpath::derive_struct_path_impl;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructPath, attributes(type_hint))]
pub fn derive_struct_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_path_impl(input).into()
}

#[proc_macro_derive(EnumPath, attributes(type_hint, enum_path))]
pub fn derive_enum_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_enum_path_impl(input).into()
}
