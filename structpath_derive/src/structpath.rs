use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    AngleBracketedGenericArguments, Attribute, Data, DeriveInput, Expr, Fields, GenericArgument,
    Ident, Lit, Meta, Type,
};

fn extract_type_hint(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        // Check if this is our type_hint attribute
        if attr.path().is_ident("type_hint") {
            // Handle different attribute syntaxes
            match &attr.meta {
                Meta::NameValue(meta_name_value) => {
                    // #[type_hint = "enum"]
                    if let Expr::Lit(expr_lit) = &meta_name_value.value {
                        if let Lit::Str(lit_str) = &expr_lit.lit {
                            return Some(lit_str.value());
                        }
                    }
                }
                Meta::List(meta_list) => {
                    // #[type_hint("enum")]
                    if let Ok(lit_str) = syn::parse2::<syn::LitStr>(meta_list.tokens.clone()) {
                        return Some(lit_str.value());
                    }
                }
                _ => return None,
            }
        }
    }
    None
}

#[derive(Debug, PartialEq)]
enum DataType {
    Basic,
    Enum,
    Struct,
}

impl DataType {
    fn new(field: &syn::Field) -> Self {
        let type_hint_opt = extract_type_hint(field.attrs.as_ref());
        match type_hint_opt {
            Some(type_hint) => match type_hint.as_str() {
                "enum" => DataType::Enum,
                "struct" => DataType::Struct,
                _ => DataType::Basic,
            },
            None => DataType::Basic,
        }
    }
}

fn is_option(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "Option")
        .unwrap_or(false)
}

/// Check if a type path is a Vec
fn is_vec(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|segment| segment.ident == "Vec")
        .unwrap_or(false)
}

fn get_angle_bracketed_inner(type_path: &syn::TypePath) -> Option<&Type> {
    type_path.path.segments.last().and_then(|segment| {
        if let syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) =
            &segment.arguments
        {
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

#[derive(Debug, PartialEq)]
enum VectorType {
    ScalarRequired,
    ScalarOptional,
    VectorRequiredElementsRequired,
    VectorRequiredElementsOptional,
    VectorOptionalElementsRequired,
    VectorOptionalElementsOptional,
    Unknown,
}

impl VectorType {
    fn new(field_type: &syn::Type) -> Self {
        match field_type {
            syn::Type::Path(type_path) => {
                if is_option(type_path) {
                    if let Some(syn::Type::Path(inner_type1)) = get_angle_bracketed_inner(type_path)
                    {
                        if is_vec(inner_type1) {
                            if let Some(syn::Type::Path(inner_type2)) =
                                get_angle_bracketed_inner(inner_type1)
                            {
                                if is_option(inner_type2) {
                                    VectorType::VectorOptionalElementsOptional
                                } else {
                                    VectorType::VectorOptionalElementsRequired
                                }
                            } else {
                                VectorType::Unknown
                            }
                        } else {
                            VectorType::ScalarOptional
                        }
                    } else {
                        VectorType::Unknown
                    }
                } else if is_vec(type_path) {
                    if let Some(syn::Type::Path(inner_type1)) = get_angle_bracketed_inner(type_path)
                    {
                        if is_option(inner_type1) {
                            VectorType::VectorRequiredElementsOptional
                        } else {
                            VectorType::VectorRequiredElementsRequired
                        }
                    } else {
                        VectorType::Unknown
                    }
                } else {
                    VectorType::ScalarRequired
                }
            }
            _ => VectorType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq)]
struct FieldInfo {
    name: Ident,
    data_type: DataType,
    vector_type: VectorType,
}

impl FieldInfo {
    fn new(field: &syn::Field) -> Self {
        let name = field.ident.clone().unwrap();
        let data_type = DataType::new(field);
        let vector_type = VectorType::new(&field.ty);
        Self {
            name,
            data_type,
            vector_type,
        }
    }
}

pub fn derive_struct_path_impl(input: DeriveInput) -> TokenStream {
    let type_name = input.ident;

    let fields: Vec<FieldInfo> = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields_named) => fields_named.named.iter().map(FieldInfo::new).collect(),
            _ => {
                return quote! {
                    compile_error!("StructPath can only be derived for structs if they have named fields");
                }
            }
        },
        Data::Enum(_) => return quote! {},
        _ => {
            return quote! {
                compile_error!("StructPath can only be derived for structs and enums");
            }
        }
    };

    let expr_final_field = fields.iter().map(|field| {
        let field_name = &field.name;
        quote! {
            stringify!(#field_name) => Ok(self.#field_name.clone().into())
        }
    });

    let expr_final_index = fields
        .iter()
        .filter(|field| {
            field.vector_type == VectorType::VectorRequiredElementsRequired
                || field.vector_type == VectorType::VectorRequiredElementsOptional
                || field.vector_type == VectorType::VectorOptionalElementsRequired
                || field.vector_type == VectorType::VectorOptionalElementsOptional
        })
        .map(|field| {
            let field_name = &field.name;
            quote! {
                stringify!(#field_name) => Ok((&self.#field_name.clone(), index).into())
            }
        });

    let expr_nested_field = fields
        .iter()
        .filter(|field| {
            field.data_type == DataType::Struct
                && (field.vector_type == VectorType::ScalarRequired
                    || field.vector_type == VectorType::ScalarOptional)
        })
        .map(|field| {
            let field_name = &field.name;
            match field.vector_type {
                VectorType::ScalarRequired => quote! {
                    stringify!(#field_name) => self.#field_name.get_value_by_path(&remaining_path)
                },
                VectorType::ScalarOptional => quote! {
                    stringify!(#field_name) => match self.#field_name.as_ref() {
                        Some(s) => s.get_value_by_path(&remaining_path),
                        None => Ok(Value::Optional(None))
                    }
                },
                _ => quote! {
                    compile_error!("This should never happen")
                },
            }
        });

    let expr_nested_index = fields
        .iter()
        .filter(|field|
            field.data_type == DataType::Struct
            && (
                field.vector_type == VectorType::VectorRequiredElementsRequired
                || field.vector_type == VectorType::VectorRequiredElementsOptional
                || field.vector_type == VectorType::VectorOptionalElementsRequired
                || field.vector_type == VectorType::VectorOptionalElementsOptional
            )
        )
        .map(|field|
            {
                let field_name = &field.name;
                match field.vector_type {
                    VectorType::VectorRequiredElementsRequired => quote! {
                        stringify!(#field_name) => self.#field_name[index].get_value_by_path(&remaining_path)
                    },
                    VectorType::VectorRequiredElementsOptional => quote! {
                        stringify!(#field_name) => match self.#field_name[index].as_ref() {
                            Some(s) => s.get_value_by_path(&remaining_path),
                            None => Ok(Value::Optional(None)),
                        }
                    },
                    VectorType::VectorOptionalElementsRequired => quote! {
                        stringify!(#field_name) => match self.#field_name.as_ref(){
                            Some(s) => s[index].get_value_by_path(&remaining_path),
                            None => Ok(Value::Optional(None)),
                        }
                    },
                    VectorType::VectorOptionalElementsOptional => quote! {
                        stringify!(#field_name) => match self.#field_name.as_ref(){
                            Some(s) => match s[index].as_ref() {
                                Some(s) => s.get_value_by_path(&remaining_path),
                                None => Ok(Value::Optional(None)),
                            },
                            None => Ok(Value::Optional(None)),
                        }
                    },
                    _ => quote! {
                        compile_error!("This should never happen")
                    }
                }
            }
        );

    quote! {

        impl StructPathTrait for #type_name {
            fn get_value_by_path(&self, path: &Path) -> Result<Value, StructPathError> {
                if path.components.len() > 1 {
                    let path_component = path.components[0].clone();
                    let remaining_path = Path {
                        components: path.components[1..].to_vec(),
                    };
                    return match path_component {
                        PathComponent::Field(field) => match field.as_str() {
                            #(#expr_nested_field,)*
                            _ => Err(StructPathError::FieldNotFound(field)),
                        },
                        PathComponent::ArrayIndex(field, index) => match field.as_str() {
                            #(#expr_nested_index,)*
                            _ => Err(StructPathError::FieldNotFound(field)),
                        },
                    }
                }

                let path_component = path.components[0].clone();

                match path_component {
                    PathComponent::Field(field) => match field.as_str() {
                        #(#expr_final_field,)*
                        _ => Err(StructPathError::FieldNotFound(field)),
                    },
                    PathComponent::ArrayIndex(field, index) => match field.as_str() {
                        #(#expr_final_index,)*
                        _ => Err(StructPathError::FieldNotFound(field)),
                    },
                }
            }

            fn get_value(&self, path: &str) -> Result<Value, StructPathError> {
                let path = Path::from_str(path);
                match path {
                    Ok(path) => self.get_value_by_path(&path),
                    Err(e) => Err(StructPathError::InvalidPath(e.to_string())),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, ItemStruct};

    #[test]
    fn test_data_type() {
        let input: ItemStruct = parse_quote! {
            pub struct MyStruct {
                my_string: String,
                #[type_hint = "enum"]
                my_enum: MyEnum,
                #[type_hint = "struct"]
                my_struct: MyStruct,
            }
        };

        match input.fields {
            Fields::Named(fields_named) => {
                assert_eq!(
                    DataType::new(&fields_named.named.get(0).unwrap()),
                    DataType::Basic
                );
                assert_eq!(
                    DataType::new(&fields_named.named.get(1).unwrap()),
                    DataType::Enum
                );
                assert_eq!(
                    DataType::new(&fields_named.named.get(2).unwrap()),
                    DataType::Struct
                );
            }
            _ => {}
        }
    }

    #[test]
    fn test_vector_type() {
        let input: ItemStruct = parse_quote! {
            pub struct MyStruct {
                scalar_req: String,
                scalar_opt: Option<String>,
                vec_req_elems_req: Vec<String>,
                vec_req_elems_opt: Vec<Option<String>>,
                vec_opt_elems_req: Option<Vec<String>>,
                vec_opt_elems_opt: Option<Vec<Option<String>>>,
            }
        };

        match input.fields {
            Fields::Named(fields_named) => {
                assert_eq!(
                    VectorType::new(&fields_named.named.get(0).unwrap().ty),
                    VectorType::ScalarRequired
                );
                assert_eq!(
                    VectorType::new(&fields_named.named.get(1).unwrap().ty),
                    VectorType::ScalarOptional
                );
                assert_eq!(
                    VectorType::new(&fields_named.named.get(2).unwrap().ty),
                    VectorType::VectorRequiredElementsRequired
                );
                assert_eq!(
                    VectorType::new(&fields_named.named.get(3).unwrap().ty),
                    VectorType::VectorRequiredElementsOptional
                );
                assert_eq!(
                    VectorType::new(&fields_named.named.get(4).unwrap().ty),
                    VectorType::VectorOptionalElementsRequired
                );
                assert_eq!(
                    VectorType::new(&fields_named.named.get(5).unwrap().ty),
                    VectorType::VectorOptionalElementsOptional
                );
            }
            _ => {}
        }
    }

    #[test]
    fn test_field_info() {
        let input: ItemStruct = parse_quote! {
            pub struct MyStruct {
                scalar_req: String,
                #[type_hint = "enum"]
                scalar_opt: Option<MyEnum>,
                #[type_hint = "struct"]
                vec_req_elems_req: Vec<MyStruct>,
                vec_req_elems_opt: Vec<Option<String>>,
                #[type_hint = "enum"]
                vec_opt_elems_req: Option<Vec<MyEnum>>,
                #[type_hint = "struct"]
                vec_opt_elems_opt: Option<Vec<Option<MyStruct>>>,
            }
        };

        match input.fields {
            Fields::Named(fields_named) => {
                let v = FieldInfo::new(&fields_named.named.get(0).unwrap());
                assert_eq!(v.name.to_string(), "scalar_req");
                assert_eq!(v.data_type, DataType::Basic);
                assert_eq!(v.vector_type, VectorType::ScalarRequired);
                let v = FieldInfo::new(&fields_named.named.get(1).unwrap());
                assert_eq!(v.name.to_string(), "scalar_opt");
                assert_eq!(v.data_type, DataType::Enum);
                assert_eq!(v.vector_type, VectorType::ScalarOptional);
                let v = FieldInfo::new(&fields_named.named.get(2).unwrap());
                assert_eq!(v.name.to_string(), "vec_req_elems_req");
                assert_eq!(v.data_type, DataType::Struct);
                assert_eq!(v.vector_type, VectorType::VectorRequiredElementsRequired);
                let v = FieldInfo::new(&fields_named.named.get(3).unwrap());
                assert_eq!(v.name.to_string(), "vec_req_elems_opt");
                assert_eq!(v.data_type, DataType::Basic);
                assert_eq!(v.vector_type, VectorType::VectorRequiredElementsOptional);
                let v = FieldInfo::new(&fields_named.named.get(4).unwrap());
                assert_eq!(v.name.to_string(), "vec_opt_elems_req");
                assert_eq!(v.data_type, DataType::Enum);
                assert_eq!(v.vector_type, VectorType::VectorOptionalElementsRequired);
                let v = FieldInfo::new(&fields_named.named.get(5).unwrap());
                assert_eq!(v.name.to_string(), "vec_opt_elems_opt");
                assert_eq!(v.data_type, DataType::Struct);
                assert_eq!(v.vector_type, VectorType::VectorOptionalElementsOptional);
            }
            _ => {}
        }
    }
}
