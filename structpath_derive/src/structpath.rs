use crate::utils::{parse_field_type, value_from_field};
use core::str::FromStr;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use structpath_types::{FieldInfo, FieldType};

fn expr_type_nested_field(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);

    match &field.r#type {
        FieldType::StructPath(inner_type_name) => {
            let inner_type = TokenStream::from_str(inner_type_name).ok()?;
            Some(quote! {
                stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
            })
        }
        FieldType::Option(mid_type) if matches!(**mid_type, FieldType::StructPath(_)) => {
            if let FieldType::StructPath(inner_type_name) = &**mid_type {
                let inner_type = TokenStream::from_str(inner_type_name).ok()?;
                Some(quote! {
                    stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_value_nested_field(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);

    match &field.r#type {
        FieldType::StructPath(_) => Some(quote! {
            stringify!(#field_name) => self.#field_name.get_value_by_path(&remaining_path)
        }),
        FieldType::Option(inner_type) if matches!(**inner_type, FieldType::StructPath(_)) => {
            Some(quote! {
                stringify!(#field_name) => match self.#field_name.as_ref() {
                    Some(s) => s.get_value_by_path(&remaining_path),
                    None => Err(::structpath::StructPathError::NullValue)
                }
            })
        }
        _ => None,
    }
}

fn expr_type_nested_array(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);
    match &field.r#type {
        FieldType::Vec(inner_type) => match &**inner_type {
            FieldType::StructPath(inner_type_name) => {
                let inner_type = TokenStream::from_str(inner_type_name).ok()?;
                Some(quote! {
                    stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
                })
            }
            FieldType::Option(inner_type2) if matches!(**inner_type2, FieldType::StructPath(_)) => {
                if let FieldType::StructPath(inner_type_name) = &**inner_type2 {
                    let inner_type = TokenStream::from_str(inner_type_name).ok()?;
                    Some(quote! {
                        stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
                    })
                } else {
                    None
                }
            }
            _ => None,
        },
        FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
            if let FieldType::Vec(ref inner_type) = **mid_type {
                match &**inner_type {
                    FieldType::StructPath(inner_type_name) => {
                        let inner_type = TokenStream::from_str(inner_type_name).ok()?;
                        Some(quote! {
                            stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
                        })
                    }
                    FieldType::Option(inner_type2)
                        if matches!(**inner_type2, FieldType::StructPath(_)) =>
                    {
                        if let FieldType::StructPath(inner_type_name) = &**inner_type2 {
                            let inner_type = TokenStream::from_str(inner_type_name).ok()?;
                            Some(quote! {
                                stringify!(#field_name) => #inner_type::get_type_by_path(&remaining_path)
                            })
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_value_nested_array(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);
    match &field.r#type {
        FieldType::Vec(inner_type) => match &**inner_type {
            FieldType::StructPath(_) => Some(quote! {
                stringify!(#field_name) => {
                    if index < self.#field_name.len() {
                        self.#field_name[index].get_value_by_path(&remaining_path)
                    } else {
                        Err(::structpath::StructPathError::IndexOutOfBounds(index))
                    }
                }
            }),
            FieldType::Option(inner_type2) if matches!(**inner_type2, FieldType::StructPath(_)) => {
                Some(quote! {
                    stringify!(#field_name) => {
                        if index < self.#field_name.len() {
                            match self.#field_name[index].as_ref() {
                                Some(s) => s.get_value_by_path(&remaining_path),
                                None => Err(::structpath::StructPathError::NullValue)
                            }
                        } else {
                            Err(::structpath::StructPathError::IndexOutOfBounds(index))
                        }
                    }
                })
            }
            _ => None,
        },
        FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
            if let FieldType::Vec(ref inner_type) = **mid_type {
                match &**inner_type {
                    FieldType::StructPath(_) => Some(quote! {
                        stringify!(#field_name) => match self.#field_name.as_ref() {
                            Some(vec) => {
                                if index < vec.len() {
                                    vec[index].get_value_by_path(&remaining_path)
                                } else {
                                    Err(::structpath::StructPathError::IndexOutOfBounds(index))
                                }
                            },
                            None => Err(::structpath::StructPathError::NullValue),
                        }
                    }),
                    FieldType::Option(inner_type2)
                        if matches!(**inner_type2, FieldType::StructPath(_)) =>
                    {
                        Some(quote! {
                            stringify!(#field_name) => match self.#field_name.as_ref() {
                                Some(vec) => {
                                    if index < vec.len() {
                                        match vec[index].as_ref() {
                                            Some(s) => s.get_value_by_path(&remaining_path),
                                            None => Err(::structpath::StructPathError::NullValue)
                                        }
                                    } else {
                                        Err(::structpath::StructPathError::IndexOutOfBounds(index))
                                    }
                                },
                                None => Err(::structpath::StructPathError::NullValue),
                            }
                        })
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_type_final_field(field: &FieldInfo) -> TokenStream {
    let field_name = format_ident!("{}", &field.name);
    let field_type = &field.r#type;
    quote! {
        stringify!(#field_name) => Ok(#field_type)
    }
}

fn expr_value_final_field(field: &FieldInfo) -> TokenStream {
    let field_name = format_ident!("{}", &field.name);
    let field_expr = value_from_field(&field.r#type, quote! { self.#field_name });
    quote! {
        stringify!(#field_name) => Ok(#field_expr)
    }
}

fn expr_type_final_array(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);
    match &field.r#type {
        FieldType::Vec(elem_type) => {
            let field_type = elem_type.as_ref();
            Some(quote! {
                stringify!(#field_name) => Ok(#field_type)
            })
        }
        FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
            if let FieldType::Vec(ref inner_type) = **mid_type {
                let elem_type = inner_type;
                Some(quote! {
                    stringify!(#field_name) => Ok(#elem_type)
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn expr_value_final_array(field: &FieldInfo) -> Option<TokenStream> {
    let field_name = format_ident!("{}", &field.name);
    match &field.r#type {
        FieldType::Vec(elem_type) => {
            let field_expr = value_from_field(elem_type, quote! { self.#field_name[index] });
            Some(quote! {
                stringify!(#field_name) => {
                    if index < self.#field_name.len() {
                        Ok(#field_expr)
                    } else {
                        Err(::structpath::StructPathError::IndexOutOfBounds(index))
                    }
                }
            })
        }
        FieldType::Option(mid_type) if matches!(**mid_type, FieldType::Vec(_)) => {
            if let FieldType::Vec(ref inner_type) = **mid_type {
                let field_expr = value_from_field(inner_type, quote! { vec[index] });
                Some(quote! {
                    stringify!(#field_name) => {
                        match self.#field_name.as_ref() {
                            Some(vec) => {
                                if index < vec.len() {
                                    Ok(#field_expr)
                                } else {
                                    Err(::structpath::StructPathError::IndexOutOfBounds(index))
                                }
                            }
                            None => Err(::structpath::StructPathError::NullValue),
                        }
                    }
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

fn trait_function(
    nested_field: impl Iterator<Item = TokenStream>,
    nested_array: impl Iterator<Item = TokenStream>,
    final_field: impl Iterator<Item = TokenStream>,
    final_array: impl Iterator<Item = TokenStream>,
) -> TokenStream {
    quote! {
        if path.components.len() > 1 {
            let path_component = path.components[0].clone();
            let remaining_path = ::structpath::Path {
                components: path.components[1..].to_vec(),
            };
            return match path_component {
                ::structpath::PathComponent::Field(field) => match field.as_str() {
                    #(#nested_field,)*
                    _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                },
                ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                    #(#nested_array,)*
                    _ => Err(::structpath::StructPathError::FieldNotFound(field)),
                },
            }                }

        let path_component = path.components[0].clone();

        match path_component {
            ::structpath::PathComponent::Field(field) => match field.as_str() {
                #(#final_field,)*
                _ => Err(::structpath::StructPathError::FieldNotFound(field)),
            },
            ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                #(#final_array,)*
                _ => Err(::structpath::StructPathError::FieldNotFound(field)),
            },
        }
    }
}

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
        _ => return quote! {},
    };

    let get_type_by_path = trait_function(
        fields.iter().filter_map(expr_type_nested_field),
        fields.iter().filter_map(expr_type_nested_array),
        fields.iter().map(expr_type_final_field),
        fields.iter().filter_map(expr_type_final_array),
    );

    let get_value_by_path = trait_function(
        fields.iter().filter_map(expr_value_nested_field),
        fields.iter().filter_map(expr_value_nested_array),
        fields.iter().map(expr_value_final_field),
        fields.iter().filter_map(expr_value_final_array),
    );

    quote! {

        impl ::structpath::StructPath for #type_name {

            fn get_type_by_path(path: &::structpath::Path) -> Result<::structpath::FieldType, ::structpath::StructPathError> {
                #get_type_by_path
            }

            fn get_value_by_path(&self, path: &::structpath::Path) -> Result<::structpath::Value, ::structpath::StructPathError> {
                #get_value_by_path
            }
        }
    }
}
