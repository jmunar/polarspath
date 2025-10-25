use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use structpath_types::indexmap::IndexMap;
use syn::{Expr, ExprLit, Lit};

pub fn derive_enum_path_impl(input: syn::DeriveInput) -> TokenStream {
    let type_name = input.ident;

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

    let variants: IndexMap<String, u32> = match input.data {
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

            // Now collect the variants
            data_enum
                .variants
                .iter()
                .enumerate()
                .map(|(index, variant)| {
                    let variant_name = variant.ident.to_string();
                    let variant_value = if let Some((_, discriminant_expr)) = &variant.discriminant
                    {
                        parse_discriminant(&variant_name, discriminant_expr)
                    } else {
                        // Use implicit discriminant (index + 1, since we require positive values)
                        (index + 1) as u32
                    };
                    (variant_name, variant_value)
                })
                .collect()
        }
        _ => return quote! {},
    };

    let variants_map = variants.iter().map(|(name, value)| {
        quote! { (#name.into(), #value) }
    });

    let variant_matches = variants.iter().enumerate().map(|(index, (name, _))| {
        let variant_ident = syn::Ident::new(name, proc_macro2::Span::call_site());
        quote! { #type_name::#variant_ident => ::polars_core::prelude::AnyValue::Enum(#index as u32, <#type_name as ::structpath::EnumPath>::mapping()) }
    });

    quote! {
        impl ::structpath::HasDataTypeOpt for #type_name {
            fn data_type_opt() -> &'static ::structpath::DataTypeOpt {
                static DATA_TYPE_OPT: ::std::sync::OnceLock<::structpath::DataTypeOpt> = ::std::sync::OnceLock::new();
                DATA_TYPE_OPT.get_or_init(|| ::structpath::DataTypeOpt::Enum(::structpath::indexmap::IndexMap::from([
                    #(#variants_map),*
                ])))
            }

            fn data_type() -> &'static ::polars_core::prelude::DataType {
                static DATA_TYPE: ::std::sync::OnceLock<::polars_core::prelude::DataType> = ::std::sync::OnceLock::new();
                DATA_TYPE.get_or_init(|| Self::data_type_opt().to_data_type())
            }
        }

        impl ::structpath::EnumPath for #type_name
        where
            #type_name: ::structpath::HasDataTypeOpt,
        {
            fn mapping() -> &'static ::std::sync::Arc<::polars_core::prelude::CategoricalMapping> {
                match <Self as ::structpath::HasDataTypeOpt>::data_type() {
                    ::polars_core::prelude::DataType::Enum(_, mapping) => mapping,
                    _ => unreachable!(),
                }
            }
        }

        impl ::structpath::IntoAnyValueWith<#type_name> for ::structpath::DataTypeOpt
        where
            #type_name: ::structpath::EnumPath,
        {
            type ChunkDataType = ::polars_core::prelude::CategoricalType;

            fn to_any_value(&self, value: &#type_name) -> ::polars_core::prelude::AnyValue {
                match self {
                    ::structpath::DataTypeOpt::Enum(_) => match value {
                        #(#variant_matches),*
                    },
                    _ => panic!("Unsupported DataTypeOpt for #type_name: {:?}", self),
                }
            }
        }
    }
}
