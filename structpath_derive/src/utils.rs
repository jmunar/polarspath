use quote::{quote, ToTokens};
use structpath_types::indexmap::IndexMap;
use structpath_types::DataTypeOpt;
use syn::PathArguments::AngleBracketed;
use syn::{
    AngleBracketedGenericArguments, Attribute, Expr, ExprLit, GenericArgument, Lit, Meta, Type,
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

fn has_type_hint_enum(attrs: &[Attribute]) -> Option<IndexMap<String, u32>> {
    for attr in attrs {
        if attr.path().is_ident("type_hint") {
            match &attr.meta {
                Meta::List(meta_list) => {
                    // Parse #[type_hint("enum", [("item1", 3), ("item2", 5)])]
                    let tokens = meta_list.tokens.clone();

                    // Parse as a tuple by wrapping the tokens in parentheses
                    let tuple_tokens = quote! { (#tokens) };
                    if let Ok(parsed) = syn::parse2::<syn::ExprTuple>(tuple_tokens) {
                        if parsed.elems.len() == 2 {
                            // First element should be "enum"
                            if let Some(Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            })) = parsed.elems.first()
                            {
                                if lit_str.value() == "enum" {
                                    // Second element should be an array of tuples
                                    if let Some(second_elem) = parsed.elems.get(1) {
                                        return Some(parse_enum_array(second_elem));
                                    }
                                }
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
    }
    None
}

fn parse_enum_array(expr: &Expr) -> IndexMap<String, u32> {
    // Parse an array like [("item1", 3), ("item2", 5)] into IndexMap<String, u32>
    let array_expr = match expr {
        Expr::Array(arr) => arr,
        _ => panic!(
            "Expected array of tuples for enum type_hint, got: {}",
            expr.to_token_stream()
        ),
    };

    let mut map = IndexMap::new();

    for elem in &array_expr.elems {
        let tuple_expr = match elem {
            Expr::Tuple(tuple) if tuple.elems.len() == 2 => tuple,
            _ => panic!(
                "Expected tuple with exactly 2 elements, got: {}",
                expr.to_token_stream()
            ),
        };

        let key = match &tuple_expr.elems[0] {
            Expr::Lit(ExprLit {
                lit: Lit::Str(s), ..
            }) => s.value(),
            _ => panic!(
                "Expected string literal as first tuple element, got: {}",
                tuple_expr.elems[0].to_token_stream()
            ),
        };

        let value = match &tuple_expr.elems[1] {
            Expr::Lit(ExprLit {
                lit: Lit::Int(i), ..
            }) => i
                .base10_parse()
                .unwrap_or_else(|_| panic!("Invalid integer literal: {}", i.to_token_stream())),
            _ => panic!(
                "Expected integer literal as second tuple element, got: {}",
                tuple_expr.elems[1].to_token_stream()
            ),
        };
        map.insert(key, value);
    }

    map
}

pub fn parse_data_type(field_type: &Type, attrs: &[Attribute]) -> DataTypeOpt {
    match field_type {
        syn::Type::Path(type_path) => match type_path.path.segments.last() {
            Some(segment) => {
                let segment_name = segment.ident.to_string();

                match segment_name.as_str() {
                    "String" => DataTypeOpt::String,
                    "i32" => {
                        if let Some(enum_values) = has_type_hint_enum(attrs) {
                            DataTypeOpt::Enum(enum_values)
                        } else {
                            DataTypeOpt::Int32
                        }
                    }
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
                    _ => {
                        if let Some(enum_values) = has_type_hint_enum(attrs) {
                            DataTypeOpt::Enum(enum_values)
                        } else if has_type_hint_struct(attrs) {
                            let type_name = type_path.to_token_stream().to_string();
                            DataTypeOpt::StructFuture(Box::leak(type_name.into_boxed_str()))
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

    #[test]
    fn test_parse_data_type() {
        let field_type: syn::Type = syn::parse_str("String").unwrap();
        let attrs = vec![];
        let data_type = parse_data_type(&field_type, &attrs);
        assert_eq!(data_type, DataTypeOpt::String);
    }

    #[test]
    fn test_parse_enum_array_basic() {
        // Test parsing [("item1", 3), ("item2", 5)]
        let array_expr: Expr = syn::parse_str("[(\"item1\", 3), (\"item2\", 5)]").unwrap();
        let result = parse_enum_array(&array_expr);
        let expected = IndexMap::from([("item1".to_string(), 3), ("item2".to_string(), 5)]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_enum_array_empty() {
        // Test parsing empty array []
        let array_expr: Expr = syn::parse_str("[]").unwrap();
        let result = parse_enum_array(&array_expr);
        let expected = IndexMap::new();
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Expected integer literal as second tuple element")]
    fn test_parse_enum_array_invalid_value() {
        // Test parsing invalid array [("item1", "not_a_number")]
        let array_expr: Expr = syn::parse_str("[(\"item1\", \"not_a_number\")]").unwrap();
        parse_enum_array(&array_expr);
    }

    #[test]
    #[should_panic(expected = "Expected array of tuples for enum type_hint")]
    fn test_parse_enum_array_not_array() {
        // Test parsing non-array expression
        let expr: Expr = syn::parse_str("\"not_an_array\"").unwrap();
        parse_enum_array(&expr);
    }
}
