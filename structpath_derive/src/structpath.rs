use crate::utils::{parse_field_type, value_from_field};
use proc_macro2::TokenStream;
use quote::quote;
use structpath_types::{FieldInfo, FieldType};

pub fn derive_struct_path_impl(input: syn::DeriveInput) -> TokenStream {
    let type_name = input.ident;

    let fields: Vec<FieldInfo> = match input.data {
        syn::Data::Struct(data_struct) if matches!(data_struct.fields, syn::Fields::Named(_)) => {
            if let syn::Fields::Named(fields_named) = data_struct.fields {
                fields_named
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.clone().unwrap();
                        let field_type = parse_field_type(&field.ty, &field.attrs);
                        FieldInfo {
                            name: field_name.to_string(),
                            r#type: field_type,
                        }
                    })
                    .collect()
            } else {
                return quote! {
                    compile_error!("StructPath can only be derived for structs with named fields");
                };
            }
        }
        _ => {
            return quote! {
                compile_error!("StructPath can only be derived for structs with named fields");
            }
        }
    };

    let expr_fields_info: Vec<TokenStream> =
        fields.iter().clone().map(|field| quote! {#field}).collect();

    let expr_final_field = fields.iter().map(|field| {
        let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        let field_expr = value_from_field(&field.r#type, quote! { self.#field_name });
        quote! {
            stringify!(#field_name) => Ok(#field_expr)
        }
    });

    let expr_final_index = fields.iter().filter_map(|field| {
        let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        match &field.r#type {
            FieldType::Vec(elem_type) => {
                let field_expr = value_from_field(elem_type, quote! { self.#field_name[index] });
                Some(quote! {
                    stringify!(#field_name) => Ok(#field_expr)
                })
            }
            FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
                if let FieldType::Vec(ref inner_type) = **mid_type {
                    let field_expr = value_from_field(inner_type, quote! { vec[index] });
                    Some(quote! {
                        stringify!(#field_name) => Ok(
                            match self.#field_name.as_ref() {
                                Some(vec) => #field_expr,
                                None => ::structpath::Value::Option(None),
                            }
                        )
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    });

    let expr_nested_field = fields.iter().filter_map(|field| {
        let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
        match &field.r#type {
            FieldType::StructPath => Some(quote! {
                stringify!(#field_name) => self.#field_name.get_value_by_path(&remaining_path)
            }),
            FieldType::Option(inner_type) if matches!(**inner_type, FieldType::StructPath) => {
                Some(quote! {
                    stringify!(#field_name) => match self.#field_name.as_ref() {
                        Some(s) => s.get_value_by_path(&remaining_path),
                        None => Ok(::structpath::Value::Option(None))
                    }
                })
            }
            _ => None,
        }
    });

    let expr_nested_index = fields
        .iter()
        .filter_map(|field| {
            let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
            match &field.r#type {
                FieldType::Vec(inner_type) => {
                    if **inner_type == FieldType::StructPath {
                        Some(
                            quote! {
                                stringify!(#field_name) => self.#field_name[index].get_value_by_path(&remaining_path)
                            }
                        )
                    } else if **inner_type == FieldType::Option(Box::new(FieldType::StructPath)) {
                        Some(
                            quote! {
                                stringify!(#field_name) => match self.#field_name[index].as_ref() {
                                    Some(s) => s.get_value_by_path(&remaining_path),
                                    None => Ok(::structpath::Value::Option(None))
                                }
                            }
                        )
                    } else {
                        None
                    }
                },
                FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
                    if let FieldType::Vec(ref inner_type) = **mid_type {
                        if **inner_type == FieldType::StructPath {
                            Some(
                                quote! {
                                    stringify!(#field_name) => match self.#field_name.as_ref() {
                                        Some(vec) => vec[index].get_value_by_path(&remaining_path),
                                        None => Ok(::structpath::Value::Option(None)),
                                    }
                                }
                            )
                        } else if **inner_type == FieldType::Option(Box::new(FieldType::StructPath)) {
                            Some(
                                quote! {
                                    stringify!(#field_name) => match self.#field_name.as_ref() {
                                        Some(vec) => match vec[index].as_ref() {
                                            Some(s) => s.get_value_by_path(&remaining_path),
                                            None => Ok(::structpath::Value::Option(None))
                                        },
                                        None => Ok(::structpath::Value::Option(None)),
                                    }
                                }
                            )
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                },
                _ => None,
            }
        });

    quote! {

        impl ::structpath::StructPath for #type_name {

            fn get_fields_info() -> ::structpath_types::FieldsInfo {
                ::structpath_types::FieldsInfo {
                    fields: vec![#(#expr_fields_info),*],
                }
            }

            fn get_value_by_path(&self, path: &::structpath::Path) -> Result<::structpath::Value, ::structpath::StructPathError> {
                if path.components.len() > 1 {
                    let path_component = path.components[0].clone();
                    let remaining_path = ::structpath::Path {
                        components: path.components[1..].to_vec(),
                    };
                    return match path_component {
                        ::structpath::PathComponent::Field(field) => match field.as_str() {
                            #(#expr_nested_field,)*
                            _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                        },
                        ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                            #(#expr_nested_index,)*
                            _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                        },
                    }
                }

                let path_component = path.components[0].clone();

                match path_component {
                    ::structpath::PathComponent::Field(field) => match field.as_str() {
                        #(#expr_final_field,)*
                        _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                    },
                    ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                        #(#expr_final_index,)*
                        _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                    },
                }
            }

            fn get_value(&self, path: &str) -> Result<::structpath::Value, ::structpath::StructPathError> {
                let path = ::structpath::Path::from_str(path);
                match path {
                    Ok(path) => self.get_value_by_path(&path),
                    Err(e) => Err(::structpath::StructPathError::InvalidPath(e.to_string())),
                }
            }
        }
    }
}
