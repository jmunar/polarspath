use quote::ToTokens;
use structpath_types::DataTypeOpt;
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

pub fn parse_data_type(field_type: &Type, attrs: &[Attribute]) -> DataTypeOpt {
    match field_type {
        syn::Type::Path(type_path) => match type_path.path.segments.last() {
            Some(segment) => {
                let segment_name = segment.ident.to_string();

                match segment_name.as_str() {
                    "String" => DataTypeOpt::String,
                    "i32" => DataTypeOpt::Int32,
                    "i64" => DataTypeOpt::Int64,
                    "f64" => DataTypeOpt::Float64,
                    "bool" => DataTypeOpt::Boolean,
                    "Vec" => {
                        let inner_type =
                            parse_data_type(get_angle_bracketed_inner(type_path).unwrap(), attrs);
                        DataTypeOpt::List(Box::new(inner_type))
                    }
                    "Option" => {
                        let inner_type =
                            parse_data_type(get_angle_bracketed_inner(type_path).unwrap(), attrs);
                        DataTypeOpt::Option(Box::new(inner_type))
                    }
                    _ if is_structpath(attrs) => {
                        let type_name = type_path.to_token_stream().to_string();
                        DataTypeOpt::StructFuture(Box::leak(type_name.into_boxed_str()))
                    }
                    _ => {
                        let type_name = type_path.to_token_stream().to_string();
                        DataTypeOpt::Object(Box::leak(type_name.into_boxed_str()))
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

    #[test]
    fn test_parse_data_type() {
        let field_type: syn::Type = syn::parse_str("String").unwrap();
        let attrs = vec![];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(data_type, DataTypeOpt::String);
    }
}
