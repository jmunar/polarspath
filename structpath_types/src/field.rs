use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug, PartialEq)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    StructPath,
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
            FieldType::StructPath => quote! { ::structpath_types::FieldType::StructPath },
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

#[derive(Debug)]
pub struct FieldsInfo {
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub name: String,
    pub r#type: FieldType,
}

impl ToTokens for FieldInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let self_name = &self.name;
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

        let field_type = FieldType::StructPath;
        let mut tokens = TokenStream::new();
        field_type.to_tokens(&mut tokens);
        assert_eq!(
            tokens.to_string(),
            ":: structpath_types :: FieldType :: StructPath"
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
