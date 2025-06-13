mod structinfo;
mod structpath;

use proc_macro::TokenStream;
use structpath::derive_struct_path_impl;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(StructPath, attributes(type_hint))]
pub fn derive_struct_path(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    derive_struct_path_impl(input).into()
}
