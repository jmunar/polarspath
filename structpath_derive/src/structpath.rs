use crate::utils::parse_data_type;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use structpath_types::{indexmap::IndexMap, DataTypeOpt, DataTypeWrapper};

pub fn derive_struct_path_impl(input: syn::DeriveInput) -> TokenStream {
    let type_name = input.ident;

    let fields: IndexMap<String, DataTypeWrapper> = match input.data {
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
        match &dtype.raw {
            DataTypeOpt::StructFuture(_) => {
                Some(
                    quote! {
                        stringify!(#field_name) => self.#field_name.get_value_by_path(&remaining_path)
                    }
                )
            }
            DataTypeOpt::Option(mid_type) => {
                if let DataTypeOpt::StructFuture(_) = mid_type.raw {
                    Some(
                        quote! {
                            stringify!(#field_name) => match self.#field_name {
                                Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
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
        match &dtype.raw {
            DataTypeOpt::List(mid_type1) => {
                match &mid_type1.raw {
                    DataTypeOpt::StructFuture(_) => {
                        Some(
                            quote! {
                                stringify!(#field_name) => {
                                    if index < self.#field_name.len() {
                                        self.#field_name[index].get_value_by_path(&remaining_path)
                                    } else {
                                        Ok(::structpath::polars_core::prelude::AnyValue::Null)
                                    }
                                }
                            }
                        )
                    }
                    DataTypeOpt::Option(mid_type2) => {
                        if let DataTypeOpt::StructFuture(_) = mid_type2.raw {
                            Some(
                                quote! {
                                    stringify!(#field_name) => {
                                        if index < self.#field_name.len() {
                                            match self.#field_name[index] {
                                                Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                                None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
                                            }
                                        } else {
                                            Ok(::structpath::polars_core::prelude::AnyValue::Null)
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
                if let DataTypeOpt::List(ref mid_type2) = mid_type1.raw {
                    match &mid_type2.raw {
                        DataTypeOpt::StructFuture(_) => {
                            Some(
                                quote! {
                                    stringify!(#field_name) => match self.#field_name {
                                        Some(ref vec) => {
                                            if index < vec.len() {
                                                vec[index].get_value_by_path(&remaining_path)
                                            } else {
                                                Ok(::structpath::polars_core::prelude::AnyValue::Null)
                                            }
                                        }
                                        None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
                                    }
                                }
                            )
                        }
                        DataTypeOpt::Option(mid_type3) => {
                            if let DataTypeOpt::StructFuture(_) = mid_type3.raw {
                                Some(
                                    quote! {
                                        stringify!(#field_name) => match self.#field_name {
                                            Some(ref vec) => {
                                                if index < vec.len() {
                                                    match vec[index] {
                                                        Some(ref struct_value) => struct_value.get_value_by_path(&remaining_path),
                                                        None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
                                                    }
                                                } else {
                                                    Ok(::structpath::polars_core::prelude::AnyValue::Null)
                                                }
                                            }
                                            None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
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
        match &dtype.raw {
            DataTypeOpt::List(_) => Some(
                quote! {
                    stringify!(#field_name) => {
                        if index < self.#field_name.len() {
                            Ok(::structpath::IntoAnyValueWith::to_any_value(field_inner_type, &self.#field_name[index]))
                        } else {
                            Ok(::structpath::polars_core::prelude::AnyValue::Null)
                        }
                    }
                }
            ),
            DataTypeOpt::Option(mid_type) if matches!(mid_type.raw, DataTypeOpt::List(_)) => Some(
                quote! {
                    stringify!(#field_name) => match &self.#field_name {
                        Some(ref vec) => {
                            if index < vec.len() {
                                Ok(::structpath::IntoAnyValueWith::to_any_value(field_inner_type, &vec[index]))
                            } else {
                                Ok(::structpath::polars_core::prelude::AnyValue::Null)
                            }
                        }
                        None => Ok(::structpath::polars_core::prelude::AnyValue::Null),
                    }
                }
            ),
            _ => None,
        }
    });

    quote! {

        impl ::structpath::HasDataTypeWrapper for #type_name {
            fn data_type_wrapper() -> &'static ::structpath::DataTypeWrapper {
                static DATA_TYPE_WRAPPER: ::std::sync::OnceLock<::structpath::DataTypeWrapper> = ::std::sync::OnceLock::new();
                DATA_TYPE_WRAPPER.get_or_init(|| ::structpath::DataTypeWrapper::new(::structpath::DataTypeOpt::Struct(
                    ::structpath::indexmap::IndexMap::from([
                        #(#fields_tokens),*
                    ])
                )))
            }
        }

        impl ::structpath::StructPath for #type_name {

            fn get_value_by_path(&self, path: &::structpath::Path) -> Result<::structpath::polars_core::prelude::AnyValue, ::structpath::DataTypeWrapperError> {
                let path_component = path.components[0].clone();

                if path.components.len() > 1 {
                    let remaining_path = ::structpath::Path {
                        components: path.components[1..].to_vec(),
                    };
                    return match path_component {
                        ::structpath::PathComponent::Field(field) => match field.as_str() {
                            #(#nested_field,)*
                            _ => Err(::structpath::DataTypeWrapperError::FieldNotFound(field)),
                        },
                        ::structpath::PathComponent::ArrayIndex(field, index) => match field.as_str() {
                            #(#nested_array,)*
                            _ => Err(::structpath::DataTypeWrapperError::FieldNotFound(field)),
                        },
                    }
                }


                match path_component {
                    ::structpath::PathComponent::Field(name) => {
                        let field_type = Self::fields_opt()
                            .get(&name)
                            .ok_or(::structpath::DataTypeWrapperError::FieldNotFound(name.to_string()))?;
                        match name.as_str() {
                            #(#final_field,)*
                            _ => Err(::structpath::DataTypeWrapperError::FieldNotFound(name)),
                        }
                    },
                    ::structpath::PathComponent::ArrayIndex(name, index) => {
                        let field_type = Self::fields_opt()
                            .get(&name)
                            .ok_or(::structpath::DataTypeWrapperError::FieldNotFound(name.to_string()))?;
                        let field_inner_type = match &field_type.raw {
                            ::structpath::DataTypeOpt::List(inner_type) => &**inner_type,
                            ::structpath::DataTypeOpt::Option(midt) if matches!(midt.raw, ::structpath::DataTypeOpt::List(_)) => {
                                if let ::structpath::DataTypeOpt::List(ref inner_type) = midt.raw {
                                    &**inner_type
                                } else {
                                    return Err(::structpath::DataTypeWrapperError::FieldNotFound(name.to_string()));
                                }
                            }
                            _ => return Err(::structpath::DataTypeWrapperError::FieldNotFound(name.to_string())),
                        };
                        match name.as_str() {
                            #(#final_array,)*
                            _ => Err(::structpath::DataTypeWrapperError::FieldNotFound(name)),
                        }
                    },
                }
            }
        }

        impl ::structpath::IntoAnyValueWith<#type_name> for ::structpath::DataTypeWrapper
        where #type_name: ::structpath::StructPath,
        {
            type ChunkDataType = ::structpath::polars_core::prelude::StructType;

            fn to_any_value(&self, value: &#type_name) -> ::structpath::polars_core::prelude::AnyValue {
                let field_defs = <#type_name as ::structpath::StructPath>::fields().clone();
                let field_values = <#type_name as ::structpath::StructPath>::fields_opt()
                    .iter()
                    .map(|(field_name, _)| ::structpath::StructPath::get_value(value, field_name).unwrap().into_static())
                    .collect::<Vec<::structpath::polars_core::prelude::AnyValue>>();
                ::structpath::polars_core::prelude::AnyValue::StructOwned(Box::new((field_values, field_defs)))
            }
        }
    }
}
