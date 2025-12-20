use polars_structpath_types::{data_type_wrapper, DataTypeOpt, DataTypeWrapper};
use quote::ToTokens;
use syn::PathArguments::AngleBracketed;
use syn::{
    parse2, AngleBracketedGenericArguments, Attribute, Expr, GenericArgument, Lit, Meta, Type,
};

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

fn has_type_hint_struct(attrs: &[Attribute]) -> bool {
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

fn has_type_hint_enum(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        // Check if this is our type_hint attribute
        if attr.path().is_ident("type_hint") {
            // Handle different attribute syntaxes
            match &attr.meta {
                Meta::NameValue(meta_name_value) => {
                    // #[type_hint = "enum"]
                    if let Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            return lit_str.value() == "enum";
                        }
                    }
                }
                Meta::List(meta_list) => {
                    // #[type_hint("enum")]
                    if let Ok(lit_str) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone()) {
                        return lit_str.value() == "enum";
                    }
                }
                _ => return false,
            }
        }
    }
    false
}

// Extract the second argument of the type_hint attribute if it is an enum
// Example: #[type_hint("enum", "sample.user.Loyalty")]
fn get_type_hint_enum(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        // Check if this is our type_hint attribute
        if attr.path().is_ident("type_hint") {
            // Handle different attribute syntaxes
            match &attr.meta {
                Meta::List(meta_list) => {
                    // #[type_hint("enum", "sample.user.Loyalty")]
                    // Parse the inner tokens as a tuple expression by wrapping in parentheses
                    use proc_macro2::{Delimiter, Group};
                    let tokens = meta_list.tokens.clone();
                    let group = Group::new(Delimiter::Parenthesis, tokens);
                    let mut wrapped_tokens = proc_macro2::TokenStream::new();
                    wrapped_tokens.extend(std::iter::once(proc_macro2::TokenTree::Group(group)));
                    if let Ok(syn::Expr::Tuple(syn::ExprTuple { elems, .. })) =
                        parse2::<syn::Expr>(wrapped_tokens)
                    {
                        let mut elems_iter = elems.iter();
                        // Get the first argument
                        if let Some(syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(first),
                            ..
                        })) = elems_iter.next()
                        {
                            if first.value() == "enum" {
                                // Get the second argument if it exists
                                if let Some(syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Str(second),
                                    ..
                                })) = elems_iter.next()
                                {
                                    return Some(second.value());
                                }
                            }
                        }
                    }
                }
                _ => return None,
            }
        }
    }
    None
}

pub fn parse_data_type(field_type: &Type, attrs: &[Attribute]) -> DataTypeWrapper {
    match field_type {
        syn::Type::Path(type_path) => match type_path.path.segments.last() {
            Some(segment) => {
                let segment_name = segment.ident.to_string();

                match segment_name.as_str() {
                    "String" => data_type_wrapper!(String),
                    "Vec" => {
                        // Special handling for Vec<u8> -> Bytes
                        if let Some(inner_type) = get_angle_bracketed_inner(type_path) {
                            if let syn::Type::Path(inner_path) = inner_type {
                                if let Some(inner_segment) = inner_path.path.segments.last() {
                                    if inner_segment.ident == "u8" {
                                        return data_type_wrapper!(Bytes);
                                    }
                                }
                            }
                            let inner_type_wrapper = parse_data_type(inner_type, attrs);
                            DataTypeWrapper::new(DataTypeOpt::List(Box::new(inner_type_wrapper)))
                        } else {
                            panic!("Vec type must have type parameters");
                        }
                    }
                    "i32" => {
                        if let Some(type_name) = get_type_hint_enum(attrs) {
                            DataTypeWrapper::new(DataTypeOpt::EnumFuture(Box::leak(
                                type_name.into_boxed_str(),
                            )))
                        } else {
                            data_type_wrapper!(Int32)
                        }
                    }
                    "i64" => data_type_wrapper!(Int64),
                    "u32" => data_type_wrapper!(UInt32),
                    "u64" => data_type_wrapper!(UInt64),
                    "f32" => data_type_wrapper!(Float32),
                    "f64" => data_type_wrapper!(Float64),
                    "bool" => data_type_wrapper!(Boolean),
                    "Option" => {
                        let inner_type =
                            parse_data_type(get_angle_bracketed_inner(type_path).unwrap(), attrs);
                        DataTypeWrapper::new(DataTypeOpt::Option(Box::new(inner_type)))
                    }
                    _ => {
                        if has_type_hint_enum(attrs) {
                            let type_name = type_path.to_token_stream().to_string();
                            DataTypeWrapper::new(DataTypeOpt::EnumFuture(Box::leak(
                                type_name.into_boxed_str(),
                            )))
                        } else if has_type_hint_struct(attrs) {
                            let type_name = type_path.to_token_stream().to_string();
                            DataTypeWrapper::new(DataTypeOpt::StructFuture(Box::leak(
                                type_name.into_boxed_str(),
                            )))
                        } else {
                            panic!(
                                "Unsupported type: {:?}",
                                type_path.to_token_stream().to_string()
                            )
                        }
                    }
                }
            }
            None => panic!(
                "Unsupported type: {:?}",
                type_path.to_token_stream().to_string()
            ),
        },
        _ => panic!(
            "Unsupported type: {:?}",
            field_type.to_token_stream().to_string()
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use polars_structpath_types::{DataTypeOpt, DataTypeWrapper};

    #[test]
    fn test_has_type_hint_struct() {
        let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! { #[type_hint("struct")] }];
        let has_type_hint_struct = has_type_hint_struct(&attrs);
        assert_eq!(has_type_hint_struct, true);
    }

    #[test]
    fn test_has_type_hint_enum() {
        let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! { #[type_hint("enum")] }];
        let has_type_hint_enum = has_type_hint_enum(&attrs);
        assert_eq!(has_type_hint_enum, true);
    }

    #[test]
    fn test_get_type_hint_enum() {
        let attrs: Vec<syn::Attribute> =
            vec![syn::parse_quote! { #[type_hint("enum", "sample.user.Loyalty")] }];
        let type_hint = get_type_hint_enum(&attrs);
        assert_eq!(type_hint, Some("sample.user.Loyalty".to_string()));
    }

    #[test]
    fn test_parse_data_type_string() {
        let field_type: syn::Type = syn::parse_str("String").unwrap();
        let attrs = vec![];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(data_type, data_type_wrapper!(String));
    }

    #[test]
    fn test_parse_data_type_i32() {
        let field_type: syn::Type = syn::parse_str("i32").unwrap();
        let attrs = vec![];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(data_type, data_type_wrapper!(Int32));
    }

    #[test]
    fn test_parse_data_type_i32_enum() {
        let field_type: syn::Type = syn::parse_str("i32").unwrap();
        let attrs = vec![syn::parse_quote! { #[type_hint("enum", "SomeEnum")] }];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(
            data_type,
            DataTypeWrapper::new(DataTypeOpt::EnumFuture(Box::leak(
                "SomeEnum".to_string().into_boxed_str()
            )))
        );
    }

    #[test]
    fn test_parse_data_type_vec_string() {
        let field_type: syn::Type = syn::parse_str("Vec<String>").unwrap();
        let attrs = vec![];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(data_type, data_type_wrapper!(List(String)));
    }

    #[test]
    fn test_parse_data_type_enum() {
        let field_type: syn::Type = syn::parse_str("SomeEnum").unwrap();
        let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! { #[type_hint("enum")] }];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(
            data_type,
            DataTypeWrapper::new(DataTypeOpt::EnumFuture(Box::leak(
                "SomeEnum".to_string().into_boxed_str()
            )))
        );
    }

    #[test]
    fn test_parse_data_type_struct() {
        let field_type: syn::Type = syn::parse_str("SomeStruct").unwrap();
        let attrs: Vec<syn::Attribute> = vec![syn::parse_quote! { #[type_hint("struct")] }];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(
            data_type,
            DataTypeWrapper::new(DataTypeOpt::StructFuture(Box::leak(
                "SomeStruct".to_string().into_boxed_str()
            )))
        );
    }

    #[test]
    fn test_parse_data_type_unknown_panics() {
        let field_type: syn::Type = syn::parse_str("Unknown").unwrap();
        let attrs = vec![];
        let result = std::panic::catch_unwind(|| parse_data_type(&field_type, &attrs));
        assert!(result.is_err());
    }
}
