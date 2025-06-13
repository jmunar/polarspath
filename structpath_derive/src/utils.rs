use proc_macro2::TokenStream;
use quote::quote;
use structpath_types::FieldType;
use syn::PathArguments::AngleBracketed;
use syn::{AngleBracketedGenericArguments, Attribute, Expr, GenericArgument, Lit, Meta, Type};

fn get_angle_bracketed_inner(type_path: &syn::TypePath) -> Option<&Type> {
    type_path.path.segments.last().and_then(|segment| {
        if let AngleBracketed(AngleBracketedGenericArguments { args, .. }) = &segment.arguments {
            args.first().and_then(|arg| {
                if let GenericArgument::Type(ty) = arg {
                    Some(ty)
                } else {
                    None
                }
            })
        } else {
            None
        }
    })
}

fn is_structpath(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        // Check if this is our type_hint attribute
        if attr.path().is_ident("type_hint") {
            // Handle different attribute syntaxes
            match &attr.meta {
                Meta::NameValue(meta_name_value) => {
                    // #[type_hint = "struct"]
                    if let Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            return lit_str.value() == "struct";
                        }
                    }
                }
                Meta::List(meta_list) => {
                    // #[type_hint("struct")]
                    if let Ok(lit_str) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone()) {
                        return lit_str.value() == "struct";
                    }
                }
                _ => return false,
            }
        }
    }
    false
}

pub fn parse_field_type(field_type: &Type, attrs: &[Attribute]) -> FieldType {
    match field_type {
        syn::Type::Path(type_path) => match type_path.path.segments.last() {
            Some(segment) => {
                let segment_name = segment.ident.to_string();

                match segment_name.as_str() {
                    "String" => FieldType::String,
                    "i64" => FieldType::Integer,
                    "f64" => FieldType::Float,
                    "bool" => FieldType::Boolean,
                    "Vec" => {
                        let inner_type =
                            parse_field_type(get_angle_bracketed_inner(type_path).unwrap(), attrs);
                        FieldType::Vec(Box::new(inner_type))
                    }
                    "Option" => {
                        let inner_type =
                            parse_field_type(get_angle_bracketed_inner(type_path).unwrap(), attrs);
                        FieldType::Option(Box::new(inner_type))
                    }
                    _ if is_structpath(attrs) => FieldType::StructPath,
                    _ => FieldType::Unknown,
                }
            }
            None => FieldType::Unknown,
        },
        _ => FieldType::Unknown,
    }
}

pub fn value_from_field(field_type: &FieldType, field_value: TokenStream) -> TokenStream {
    match field_type {
        FieldType::String => quote! {
            ::structpath::Value::String(#field_value.clone())
        },
        FieldType::Integer => quote! {
            ::structpath::Value::Integer(#field_value)
        },
        FieldType::Float => quote! {
            ::structpath::Value::Float(#field_value)
        },
        FieldType::Boolean => quote! {
            ::structpath::Value::Boolean(#field_value)
        },
        FieldType::StructPath => quote! {
            ::structpath::Value::Boxed(Box::new(#field_value.clone()))
        },
        FieldType::Unknown => quote! {
            ::structpath::Value::Boxed(Box::new(#field_value.clone()))
        },
        FieldType::Vec(_) => quote! {
            ::structpath::Value::Vec(Box::new(#field_value.clone()))
        },
        FieldType::Option(inner) => {
            let inner_value = value_from_field(inner, quote! { t });
            match inner.as_ref() {
                FieldType::String
                | FieldType::StructPath
                | FieldType::Unknown
                | FieldType::Vec(_)
                | FieldType::Option(_) => quote! {
                    ::structpath::Value::Option(#field_value.as_ref().map(|t| Box::new(#inner_value)))
                },
                _ => quote! {
                    ::structpath::Value::Option(#field_value.map(|t| Box::new(#inner_value)))
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_field() {
        let field_type = FieldType::String;
        let field_value = quote! { test };
        let value = value_from_field(&field_type, field_value);
        assert_eq!(
            value.to_string(),
            quote! { ::structpath::Value::String(test.clone()) }.to_string()
        );
    }
}
