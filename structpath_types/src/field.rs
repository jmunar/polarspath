use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    StructPathUnknown(String),
    StructPath(String, Vec<FieldInfo>),
    Option(Box<FieldType>),
    Vec(Box<FieldType>),
    Unknown,
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FieldType::String => quote! { ::structpath_types::FieldType::String },
            FieldType::Integer => quote! { ::structpath_types::FieldType::Integer },
            FieldType::Float => quote! { ::structpath_types::FieldType::Float },
            FieldType::Boolean => quote! { ::structpath_types::FieldType::Boolean },
            FieldType::StructPathUnknown(struct_name) => {
                let struct_type = TokenStream::from_str(struct_name).ok().unwrap();
                quote! {
                    ::structpath_types::FieldType::StructPath(
                        #struct_name.to_string(),
                        #struct_type::fields().to_vec()
                    )
                }
            }
            FieldType::StructPath(struct_name, fields) => {
                quote! {
                    ::structpath_types::FieldType::StructPath(
                        #struct_name.to_string(),
                        vec![
                            #(#fields),*
                        ]
                    )
                }
            }
            FieldType::Option(inner) => {
                quote! { ::structpath_types::FieldType::Option(Box::new(#inner)) }
            }
            FieldType::Vec(inner) => {
                quote! { ::structpath_types::FieldType::Vec(Box::new(#inner)) }
            }
            FieldType::Unknown => quote! { ::structpath_types::FieldType::Unknown },
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
    pub name: String,
    pub r#type: FieldType,
}

impl FieldInfo {
    pub fn new(name: &str, r#type: FieldType) -> Self {
        Self {
            name: name.to_string(),
            r#type,
        }
    }
}

impl ToTokens for FieldInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let self_name = format_ident!("{}", &self.name);
        let self_type = &self.r#type;
        tokens.extend(quote! {
            ::structpath_types::FieldInfo {
                name: stringify!(#self_name).to_string(),
                r#type: #self_type,
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_type_to_tokens() {
        let field_type = FieldType::String;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: String"
        );

        let field_type = FieldType::Integer;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: Integer"
        );

        let field_type = FieldType::Float;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: Float"
        );

        let field_type = FieldType::Boolean;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: Boolean"
        );

        let field_type = FieldType::StructPathUnknown("MyStruct".to_string());
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: StructPath (\"MyStruct\" . to_string () , MyStruct :: fields () . to_vec ())"
        );

        let field_type = FieldType::StructPath(
            "MyStruct".to_string(),
            vec![FieldInfo {
                name: "f_string".to_string(),
                r#type: FieldType::String,
            }],
        );
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: StructPath (\"MyStruct\" . to_string () , vec ! [:: structpath_types :: FieldInfo { name : stringify ! (f_string) . to_string () , r#type : :: structpath_types :: FieldType :: String , }])"
        );

        let field_type = FieldType::Option(Box::new(FieldType::String));
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), ":: structpath_types :: FieldType :: Option (Box :: new (:: structpath_types :: FieldType :: String))");

        let field_type = FieldType::Vec(Box::new(FieldType::String));
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(tokens.to_string(), ":: structpath_types :: FieldType :: Vec (Box :: new (:: structpath_types :: FieldType :: String))");

        let field_type = FieldType::Unknown;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: Unknown"
        );
    }
}
