use crate::utils::parse_data_type;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use structpath_types::{indexmap::IndexMap, DataTypeOpt};

pub fn derive_struct_path_impl(input: syn::DeriveInput) -> TokenStream {
    let type_name = input.ident;

    let fields: IndexMap<String, DataTypeOpt> = match input.data {
        syn::Data::Struct(data_struct) if matches!(data_struct.fields, syn::Fields::Named(_)) => {
            if let syn::Fields::Named(fields_named) = data_struct.fields {
                fields_named
                    .named
                    .iter()
                    .map(|field| {
                        let field_name = field.ident.as_ref().unwrap().to_string();
                        let field_type = parse_data_type(&field.ty, &field.attrs);
                        (field_name, field_type)
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

    let fields_tokens = fields.iter().map(|(name, dtype)| {
        let field_name = format_ident!("{}", name.to_string());
        quote! {
            (stringify!(#field_name).into(), #dtype)
        }
    });

    // Elements in the implementation of get_value_by_path()
    let nested_field = fields.iter().filter_map(|(name, dtype)| {
        let field_name = format_ident!("{}", &name.to_string());
        match &dtype {
            DataTypeOpt::StructFuture(_) => {
                Some(
                    quote! {
                        stringify!(#field_name) => self.#field_name.get_value_by_path(&remaining_path)
                    }
                )
            }
            DataTypeOpt::Option(mid_type) => {
                if let DataTypeOpt::StructFuture(_) = **mid_type {
                    Some(
                        quote! {
                            stringify!(#field_name) => match self.#field_name {
                                Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                None => Ok(::polars_core::prelude::AnyValue::Null),
                            }
                        }
                    )
                } else {
                    None
                }
            }
            _ => None,
        }
    });

    let nested_array = fields.iter().filter_map(|(name, dtype)| {
        let field_name = format_ident!("{}", &name.to_string());
        match &dtype {
            DataTypeOpt::List(mid_type1) => {
                match &**mid_type1 {
                    DataTypeOpt::StructFuture(_) => {
                        Some(
                            quote! {
                                stringify!(#field_name) => {
                                    if index < self.#field_name.len() {
                                        self.#field_name[index].get_value_by_path(&remaining_path)
                                    } else {
                                        Ok(::polars_core::prelude::AnyValue::Null)
                                    }
                                }
                            }
                        )
                    }
                    DataTypeOpt::Option(mid_type2) => {
                        if let DataTypeOpt::StructFuture(_) = **mid_type2 {
                            Some(
                                quote! {
                                    stringify!(#field_name) => {
                                        if index < self.#field_name.len() {
                                            match self.#field_name[index] {
                                                Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                                None => Ok(::polars_core::prelude::AnyValue::Null),
                                            }
                                        } else {
                                            Ok(::polars_core::prelude::AnyValue::Null)
                                        }
                                    }
                                }
                            )
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
            DataTypeOpt::Option(mid_type1) => {
                if let DataTypeOpt::List(ref mid_type2) = **mid_type1 {
                    match &**mid_type2 {
                        DataTypeOpt::StructFuture(_) => {
                            Some(
                                quote! {
                                    stringify!(#field_name) => match self.#field_name {
                                        Some(ref vec) => {
                                            if index < vec.len() {
                                                vec[index].get_value_by_path(&remaining_path)
                                            } else {
                                                Ok(::polars_core::prelude::AnyValue::Null)
                                            }
                                        }
                                        None => Ok(::polars_core::prelude::AnyValue::Null),
                                    }
                                }
                            )
                        }
                        DataTypeOpt::Option(mid_type3) => {
                            if let DataTypeOpt::StructFuture(_) = **mid_type3 {
                                Some(
                                    quote! {
                                        stringify!(#field_name) => match self.#field_name {
                                            Some(ref vec) => {
                                                if index < vec.len() {
                                                    match vec[index] {
                                                        Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                                        None => Ok(::polars_core::prelude::AnyValue::Null),
                                                    }
                                                } else {
                                                    Ok(::polars_core::prelude::AnyValue::Null)
                                                }
                                            }
                                            None => Ok(::polars_core::prelude::AnyValue::Null),
                                        }
                                    }
                                )
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
    });

    let final_field = fields.iter().map(|(name, _)| {
        let field_name = format_ident!("{}", &name.to_string());
        quote! {
            stringify!(#field_name) => Ok(::structpath::IntoAnyValueWith::to_any_value(field_type, &self.#field_name))
        }
    });

    let final_array = fields.iter().filter_map(|(name, dtype)| {
        let field_name = format_ident!("{}", &name.to_string());
        match &dtype {
            DataTypeOpt::List(_) => Some(
                quote! {
                    stringify!(#field_name) => {
                        if index < self.#field_name.len() {
                            Ok(::structpath::IntoAnyValueWith::to_any_value(field_inner_type, &self.#field_name[index]))
                        } else {
                            Ok(::polars_core::prelude::AnyValue::Null)
                        }
                    }
                }
            ),
            DataTypeOpt::Option(mid_type) if matches!(**mid_type, DataTypeOpt::List(_)) => Some(
                quote! {
                    stringify!(#field_name) => match &self.#field_name {
                        Some(ref vec) => {
                            if index < vec.len() {
                                Ok(::structpath::IntoAnyValueWith::to_any_value(field_inner_type, &vec[index]))
                            } else {
                                Ok(::polars_core::prelude::AnyValue::Null)
                            }
                        }
                        None => Ok(::polars_core::prelude::AnyValue::Null),
                    }
                }
            ),
            _ => None,
        }
    });

    quote! {

        impl ::structpath::HasDataTypeOpt for #type_name {
            fn data_type_opt() -> &'static ::structpath::DataTypeOpt {
                static DATA_TYPE_OPT: ::std::sync::OnceLock<::structpath::DataTypeOpt> = ::std::sync::OnceLock::new();
                DATA_TYPE_OPT.get_or_init(|| ::structpath::DataTypeOpt::Struct(<Self as ::structpath::StructPath>::fields_opt().clone()))
            }

            fn data_type() -> &'static ::polars_core::prelude::DataType {
                static DATA_TYPE: ::std::sync::OnceLock<::polars_core::prelude::DataType> = ::std::sync::OnceLock::new();
                DATA_TYPE.get_or_init(|| Self::data_type_opt().to_data_type())
            }
        }

        impl ::structpath::StructPath for #type_name {
            fn fields_opt() -> &'static ::structpath::indexmap::IndexMap<String, ::structpath::DataTypeOpt> {
                static FIELDS_OPT: ::std::sync::OnceLock<::structpath::indexmap::IndexMap<String, ::structpath::DataTypeOpt>> = ::std::sync::OnceLock::new();
                FIELDS_OPT.get_or_init(||
                    ::structpath::indexmap::IndexMap::from([
                        #(#fields_tokens),*
                    ])
                )
            }

            fn fields() -> &'static [::polars_core::prelude::Field] {
                static FIELDS: ::std::sync::OnceLock<Vec<::polars_core::prelude::Field>> = ::std::sync::OnceLock::new();
                FIELDS
                    .get_or_init(|| {
                        Self::fields_opt()
                            .iter()
                            .map(|(field_name, field_type)| {
                                ::polars_core::prelude::Field::new(field_name.into(), field_type.to_data_type())
                            })
                            .collect()
                    })
                    .as_slice()
            }

            fn get_value_by_path(&self, path: &::structpath::Path) -> Result<::polars_core::prelude::AnyValue, ::structpath::DataTypeOptError> {
                if path.components.len() > 1 {
                    let path_component = path.components[0].clone();
                    let remaining_path = ::structpath::Path {
                        components: path.components[1..].to_vec(),
                    };
                    return match path_component {
                        ::structpath::PathComponent::Field(field) => match field.as_str() {
                            #(#nested_field,)*
                            _ => Err(::structpath::DataTypeOptError::FieldNotFound(field)),
                        },
                        ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                            #(#nested_array,)*
                            _ => Err(::structpath::DataTypeOptError::FieldNotFound(field)),
                        },
                    }                }

                let path_component = path.components[0].clone();

                match path_component {
                    ::structpath::PathComponent::Field(name) => {
                        let field_type = Self::fields_opt()
                            .get(&name)
                            .ok_or(::structpath::DataTypeOptError::FieldNotFound(name.to_string()))?;
                        match name.as_str() {
                            #(#final_field,)*
                            _ => Err(::structpath::DataTypeOptError::FieldNotFound(name)),
                        }
                    },
                    ::structpath::PathComponent::ArrayIndex(name, index) => {
                        let field_type = Self::fields_opt()
                            .get(&name)
                            .ok_or(::structpath::DataTypeOptError::FieldNotFound(name.to_string()))?;
                        let field_inner_type = match field_type {
                            ::structpath::DataTypeOpt::List(inner_type) => &**inner_type,
                            ::structpath::DataTypeOpt::Option(mid_ty) if matches!(**mid_ty, ::structpath::DataTypeOpt::List(_)) => {
                                if let ::structpath::DataTypeOpt::List(inner_type) = &**mid_ty {
                                    inner_type
                                } else {
                                    return Err(::structpath::DataTypeOptError::FieldNotFound(name.to_string()));
                                }
                            }
                            _ => return Err(::structpath::DataTypeOptError::FieldNotFound(name.to_string())),
                        };
                        match name.as_str() {
                            #(#final_array,)*
                            _ => Err(::structpath::DataTypeOptError::FieldNotFound(name)),
                        }
                    },
                }
            }
        }

        impl ::structpath::IntoAnyValueWith<#type_name> for ::structpath::DataTypeOpt
        where #type_name: ::structpath::StructPath,
        {
            type ChunkDataType = ::polars_core::prelude::StructType;

            fn to_any_value(&self, value: &#type_name) -> ::polars_core::prelude::AnyValue {
                let field_defs = <#type_name as ::structpath::StructPath>::fields().to_vec();
                let field_values = <#type_name as ::structpath::StructPath>::fields_opt()
                    .iter()
                    .map(|(field_name, _)| ::structpath::StructPath::get_value(value, field_name).unwrap().into_static())
                    .collect::<Vec<::polars_core::prelude::AnyValue>>();
                ::polars_core::prelude::AnyValue::StructOwned(Box::new((field_values, field_defs)))
            }
        }
    }
}
