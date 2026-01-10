use proc_macro2::TokenStream;
use quote::quote;

/// Implementation function for the `StructPath` derive macro.
///
/// This function processes a struct definition and generates code that calls
/// `impl_struct_buffer!` from `polars-structpath-types` to create Arrow buffer
/// implementations for the struct.
///
/// # Panics
///
/// - If the input is not a struct
/// - If the struct does not have named fields (e.g., tuple structs or unit structs)
///
/// # Generated Code
///
/// The function generates code in the form:
/// ```rust,ignore
/// polars_structpath_types::impl_struct_buffer!(StructName, [
///     (field1, Type1),
///     (field2, Type2),
///     ...
/// ]);
/// ```
pub fn derive_struct_path_impl(input: syn::DeriveInput) -> TokenStream {
    let struct_name = input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields) => &fields.named,
            _ => panic!("StructPath can only be derived for structs with named fields"),
        },
        _ => panic!("StructPath can only be derived for structs"),
    };

    let mut field_entries = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().expect("Field must have a name");
        let field_type = &field.ty;

        field_entries.push(quote! {
            (#field_name, #field_type)
        });
    }

    quote! {
        polars_structpath_types::impl_struct_buffer!(#struct_name, [#(#field_entries),*]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "StructPath can only be derived for structs")]
    fn test_derive_struct_path_not_struct_path_panics() {
        let input: syn::DeriveInput = syn::parse_quote! {
            enum TestEnum {
                Variant1,
            }
        };
        derive_struct_path_impl(input);
    }

    #[test]
    #[should_panic(expected = "StructPath can only be derived for structs with named fields")]
    fn test_derive_struct_path_not_named_fields_panics() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct TestStruct(i32);
        };
        derive_struct_path_impl(input);
    }

    #[test]
    fn test_derive_struct_path_ok() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct TestStruct {
                field1: i32,
            }
        };
        let output = derive_struct_path_impl(input);
        assert_eq!(
            output.to_string(),
            "polars_structpath_types :: impl_struct_buffer ! (TestStruct , [(field1 , i32)]) ;"
        );
    }
}
