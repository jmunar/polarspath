use crate::string::{
    identifier_camel_case_to_snake_case, identifier_camel_case_to_upper_snake_case,
    identifier_snake_case_to_camel_case,
};
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use structpath_types::indexmap::IndexMap;
use syn::{parse2, Attribute, Expr, ExprLit, Lit, LitStr, Meta};

// Parse the enum_path attribute to get the case conversion function name
fn get_case_conversion(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("enum_path") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    // Try to parse as a single identifier: #[enum_path(camel_case_to_upper_snake_case)]
                    if let Ok(ident) = parse2::<syn::Ident>(meta_list.tokens.clone()) {
                        return Some(ident.to_string());
                    }
                    // Try to parse as a string literal: #[enum_path("camel_case_to_upper_snake_case")]
                    if let Ok(lit_str) = parse2::<LitStr>(meta_list.tokens.clone()) {
                        return Some(lit_str.value());
                    }
                    // Try to parse as assignment: #[enum_path(case = "camel_case_to_upper_snake_case")]
                    if let Ok(expr_assign) = parse2::<syn::ExprAssign>(meta_list.tokens.clone()) {
                        if let (
                            syn::Expr::Path(expr_path),
                            syn::Expr::Lit(syn::ExprLit {
                                lit: syn::Lit::Str(lit_str),
                                ..
                            }),
                        ) = (&*expr_assign.left, &*expr_assign.right)
                        {
                            if expr_path.path.is_ident("case") {
                                return Some(lit_str.value());
                            }
                        }
                    }
                }
                Meta::NameValue(meta_name_value) => {
                    // #[enum_path = "camel_case_to_upper_snake_case"]
                    if let Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
                _ => {}
            }
        }
    }
    None
}

// Apply case conversion to a string based on the function name
fn apply_case_conversion(s: &str, case_fn: &str) -> String {
    match case_fn {
        "camel_case_to_snake_case" | "identifier_camel_case_to_snake_case" => {
            identifier_camel_case_to_snake_case(s)
        }
        "camel_case_to_upper_snake_case" | "identifier_camel_case_to_upper_snake_case" => {
            identifier_camel_case_to_upper_snake_case(s)
        }
        "snake_case_to_camel_case" | "identifier_snake_case_to_camel_case" => {
            identifier_snake_case_to_camel_case(s)
        }
        _ => {
            panic!(
                "Unknown case conversion function: {}. Supported: camel_case_to_snake_case, camel_case_to_upper_snake_case, snake_case_to_camel_case",
                case_fn
            );
        }
    }
}

pub fn derive_enum_path_impl(input: syn::DeriveInput) -> TokenStream {
    let type_name = input.ident;

    // Parse case conversion attribute
    let case_conversion = get_case_conversion(&input.attrs);

    // Helper function to parse and validate discriminant
    fn parse_discriminant(variant_name: &str, discriminant_expr: &Expr) -> u32 {
        match discriminant_expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) => i.base10_parse().unwrap_or_else(|_| {
                panic!(
                    "Invalid integer literal for variant '{}': {}",
                    variant_name,
                    i.to_token_stream()
                )
            }),
            _ => panic!(
                "Enum discriminant for variant '{}' must be an integer literal, got: {}",
                variant_name,
                discriminant_expr.to_token_stream()
            ),
        }
    }

    let (variants, original_variants): (IndexMap<String, u32>, Vec<String>) = match input.data {
        syn::Data::Enum(data_enum) => {
            // First check that all variants are unit variants
            for variant in &data_enum.variants {
                match variant.fields {
                    syn::Fields::Unit => {
                        // Variant is valid, continue
                    }
                    _ => {
                        return quote! {
                            compile_error!("EnumPath can only be derived for enums with unit variants");
                        };
                    }
                }
            }

            // Collect original variant names
            let original_variants: Vec<String> = data_enum
                .variants
                .iter()
                .map(|variant| variant.ident.to_string())
                .collect();

            // Now collect the variants with case conversion applied
            let variants: IndexMap<String, u32> = data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    let original_variant_name = variant.ident.to_string();
                    // Apply case conversion if specified
                    let variant_name = if let Some(ref case_fn) = case_conversion {
                        apply_case_conversion(&original_variant_name, case_fn)
                    } else {
                        original_variant_name.clone()
                    };
                    let variant_value = if let Some((_, discriminant_expr)) = &variant.discriminant
                    {
                        parse_discriminant(&original_variant_name, discriminant_expr)
                    } else {
                        // Use implicit discriminant (index + 1, since we require positive values)
                        (index + 1) as u32
                    };
                    (variant_name, variant_value)
                })
                .collect();

            (variants, original_variants)
        }
        _ => return quote! {},
    };

    let variants_map = variants.iter().map(|(name, value)| {
        quote! { (#name.into(), #value) }
    });

    let variant_matches = variants.iter().enumerate().map(|(index, (converted_name, _))| {
        // Find the original variant name that corresponds to this converted name
        let original_name = if let Some(ref case_fn) = case_conversion {
            // Find which original variant name, when converted, matches this converted_name
            original_variants
                .iter()
                .find(|orig| apply_case_conversion(orig, case_fn) == *converted_name)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not find original variant for converted name: {}",
                        converted_name
                    )
                })
        } else {
            converted_name
        };
        let variant_ident = syn::Ident::new(original_name, proc_macro2::Span::call_site());
        quote! { #type_name::#variant_ident => ::structpath::polars_core::prelude::AnyValue::Enum(#index as u32, <#type_name as ::structpath::EnumPath>::mapping()) }
    });

    quote! {
        impl ::structpath::HasDataTypeWrapper for #type_name {
            fn data_type_wrapper() -> &'static ::structpath::DataTypeWrapper {
                static DATA_TYPE_WRAPPER: ::std::sync::OnceLock<::structpath::DataTypeWrapper> = ::std::sync::OnceLock::new();
                DATA_TYPE_WRAPPER.get_or_init(|| ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Enum(::structpath::EnumOptInfo::from_iter([
                    #(#variants_map),*
                ]))))
            }
        }

        impl ::structpath::EnumPath for #type_name {}

        impl ::structpath::IntoAnyValueWith<#type_name> for ::structpath::DataTypeWrapper
        where
            #type_name: ::structpath::EnumPath,
        {
            type ChunkDataType = ::structpath::polars_core::prelude::CategoricalType;

            fn to_any_value(&self, value: &#type_name) -> ::structpath::polars_core::prelude::AnyValue {
                match &self.raw {
                    ::structpath::DataTypeOpt::Enum(_) => match value {
                        #(#variant_matches),*
                    },
                    _ => panic!("Unsupported DataTypeWrapper for #type_name: {:?}", self),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_case_conversion_identifier() {
        let attrs: Vec<syn::Attribute> =
            vec![syn::parse_quote! { #[enum_path(camel_case_to_upper_snake_case)] }];
        let result = get_case_conversion(&attrs);
        assert_eq!(result, Some("camel_case_to_upper_snake_case".to_string()));
    }

    #[test]
    fn test_get_case_conversion_string_literal() {
        let attrs: Vec<syn::Attribute> =
            vec![syn::parse_quote! { #[enum_path("camel_case_to_upper_snake_case")] }];
        let result = get_case_conversion(&attrs);
        assert_eq!(result, Some("camel_case_to_upper_snake_case".to_string()));
    }

    #[test]
    fn test_get_case_conversion_name_value() {
        let attrs: Vec<syn::Attribute> =
            vec![syn::parse_quote! { #[enum_path(case = "camel_case_to_upper_snake_case")] }];
        let result = get_case_conversion(&attrs);
        assert_eq!(result, Some("camel_case_to_upper_snake_case".to_string()));
    }

    #[test]
    fn test_get_case_conversion_none() {
        let attrs: Vec<syn::Attribute> = vec![];
        let result = get_case_conversion(&attrs);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_case_conversion_other_attribute() {
        let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! { #[other_attr] }];
        let result = get_case_conversion(&attrs);
        assert_eq!(result, None);
    }

    #[test]
    fn test_apply_case_conversion_camel_to_snake() {
        // Test with lowercase first character
        let result = apply_case_conversion("helloWorld", "camel_case_to_snake_case");
        assert_eq!(result, "hello_world");

        // Test with uppercase first character (no leading underscore)
        let result2 = apply_case_conversion("HelloWorld", "camel_case_to_snake_case");
        assert_eq!(result2, "hello_world");

        // Test with all uppercase (should just lowercase, no underscores)
        let result3 = apply_case_conversion("HELLO", "camel_case_to_snake_case");
        assert_eq!(result3, "hello");
    }

    #[test]
    fn test_apply_case_conversion_camel_to_upper_snake() {
        // Test with lowercase first character
        let result = apply_case_conversion("helloWorld", "camel_case_to_upper_snake_case");
        assert_eq!(result, "HELLO_WORLD");

        // Test with uppercase first character (no leading underscore)
        let result2 = apply_case_conversion("HelloWorld", "camel_case_to_upper_snake_case");
        assert_eq!(result2, "HELLO_WORLD");

        // Test with all uppercase
        let result3 = apply_case_conversion("HELLO", "camel_case_to_upper_snake_case");
        assert_eq!(result3, "HELLO");
    }

    #[test]
    fn test_apply_case_conversion_snake_to_camel() {
        let result = apply_case_conversion("hello_world", "snake_case_to_camel_case");
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_apply_case_conversion_identifier_prefixes() {
        // Test that identifier_ prefix also works
        assert_eq!(
            apply_case_conversion("helloWorld", "identifier_camel_case_to_snake_case"),
            "hello_world"
        );
        assert_eq!(
            apply_case_conversion("helloWorld", "identifier_camel_case_to_upper_snake_case"),
            "HELLO_WORLD"
        );
        assert_eq!(
            apply_case_conversion("hello_world", "identifier_snake_case_to_camel_case"),
            "HelloWorld"
        );
    }

    #[test]
    #[should_panic(expected = "Unknown case conversion function")]
    fn test_apply_case_conversion_invalid() {
        apply_case_conversion("HelloWorld", "invalid_function");
    }

    #[test]
    fn test_derive_enum_path_basic() {
        let input: syn::DeriveInput = syn::parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2,
                Variant3,
            }
        };
        let output = derive_enum_path_impl(input);
        // Just check that it compiles and produces output
        assert!(!output.is_empty());
    }

    #[test]
    fn test_derive_enum_path_with_case_conversion() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[enum_path(camel_case_to_upper_snake_case)]
            enum TestEnum {
                VariantOne,
                VariantTwo,
                VariantThree,
            }
        };
        let output = derive_enum_path_impl(input);
        // Just check that it compiles and produces output
        assert!(!output.is_empty());
    }

    #[test]
    fn test_derive_enum_path_with_explicit_discriminants() {
        let input: syn::DeriveInput = syn::parse_quote! {
            enum TestEnum {
                Variant1 = 10,
                Variant2 = 20,
                Variant3 = 30,
            }
        };
        let output = derive_enum_path_impl(input);
        // Just check that it compiles and produces output
        assert!(!output.is_empty());
    }

    #[test]
    fn test_derive_enum_path_with_string_attribute() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[enum_path("camel_case_to_snake_case")]
            enum TestEnum {
                VariantOne,
                VariantTwo,
            }
        };
        let output = derive_enum_path_impl(input);
        // Just check that it compiles and produces output
        assert!(!output.is_empty());
    }

    #[test]
    fn test_derive_enum_path_with_name_value_attribute() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[enum_path(case = "snake_case_to_camel_case")]
            enum TestEnum {
                variant_one,
                variant_two,
            }
        };
        let output = derive_enum_path_impl(input);
        // Just check that it compiles and produces output
        assert!(!output.is_empty());
    }
}
