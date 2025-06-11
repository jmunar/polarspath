mod structinfo;
mod structpath;

use proc_macro::TokenStream;
use structinfo::derive_struct_info_impl;
use structpath::derive_struct_path_impl;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructPath, attributes(type_hint))]
pub fn derive_struct_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_path_impl(input).into()
}

#[proc_macro_derive(StructInfo, attributes(type_hint))]
pub fn derive_struct_info(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_info_impl(input).into()
}
