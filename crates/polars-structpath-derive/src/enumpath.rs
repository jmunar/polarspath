use proc_macro2::TokenStream;
use quote::quote;

/// Implementation function for the `EnumPath` derive macro.
///
/// This function processes an enum definition and generates code that calls
/// `impl_enum_buffer!` from `polars_structpath` to create Arrow buffer
/// implementations for the enum.
///
/// # Panics
///
/// - If the input is not an enum
/// - If any enum variant is not a unit variant (has fields)
/// - If any enum variant does not have an explicit discriminant value
/// - If a discriminant is not a literal expression
///
/// # Generated Code
///
/// The function generates code in the form:
/// ```rust,ignore
/// ::polars_structpath::impl_enum_buffer!(EnumName, [
///     (Variant1, 1),
///     (Variant2, 2),
///     ...
/// ]);
/// ```
pub fn derive_enum_path_impl(input: syn::DeriveInput) -> TokenStream {
    let enum_name = input.ident;

    let variants = match &input.data {
        syn::Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumPath can only be derived for enums"),
    };

    let mut variant_data = Vec::new();

    for variant in variants {
        if !matches!(variant.fields, syn::Fields::Unit) {
            panic!("EnumPath can only be derived for unit enums");
        }

        let variant_name = &variant.ident;

        let discriminant = match &variant.discriminant {
            Some((_, expr)) => {
                if let syn::Expr::Lit(expr_lit) = expr {
                    quote! { #expr_lit }
                } else {
                    panic!("Discriminant must be a literal");
                }
            }
            None => panic!("All variants must have explicit discriminants"),
        };

        variant_data.push(quote! {
            (#variant_name, #discriminant)
        });
    }

    quote! {
        ::polars_structpath::impl_enum_buffer!(#enum_name, [#(#variant_data),*]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "EnumPath can only be derived for enums")]
    fn test_derive_enum_path_not_enum_path_panics() {
        let input: syn::DeriveInput = syn::parse_quote! {
            struct TestStruct {
                field1: i32,
            }
        };
        derive_enum_path_impl(input);
    }

    #[test]
    #[should_panic(expected = "EnumPath can only be derived for unit enums")]
    fn test_derive_enum_path_not_unit_enums_panics() {
        let input: syn::DeriveInput = syn::parse_quote! {
            enum TestEnum {
                VariantOne(i32),
            }
        };
        derive_enum_path_impl(input);
    }

    #[test]
    fn test_derive_enum_path_ok() {
        let input: syn::DeriveInput = syn::parse_quote! {
            enum TestEnum {
                Variant1 = 10,
                Variant2 = 20,
            }
        };
        let output = derive_enum_path_impl(input);
        // Convert to string and check that it contains the expected text
        let output_str = output.to_string();
        assert_eq!(
            output_str,
            ":: polars_structpath :: impl_enum_buffer ! (TestEnum , [(Variant1 , 10) , (Variant2 , 20)]) ;"
        );
    }
}
