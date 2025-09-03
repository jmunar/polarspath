use polars_core::prelude::{DataType, Field};
use structpath_types::FieldType;

pub fn field_type_to_data_type(path_type: FieldType) -> DataType {
    match path_type {
        FieldType::String => DataType::String,
        FieldType::Integer => DataType::Int64,
        FieldType::Float => DataType::Float64,
        FieldType::Boolean => DataType::Boolean,
        FieldType::StructPath(_, fields) => {
            let polar_fields = fields
                .iter()
                .map(|field| {
                    Field::new(
                        field.name.clone().into(),
                        field_type_to_data_type(field.r#type.clone()),
                    )
                })
                .collect::<Vec<Field>>();
            DataType::Struct(polar_fields)
        }
        FieldType::Option(inner_type) => field_type_to_data_type(*inner_type),
        FieldType::Vec(inner_type) => {
            let inner_data_type = field_type_to_data_type(*inner_type);
            DataType::List(Box::new(inner_data_type))
        }
        _ => panic!("Unsupported type: {:?}", path_type),
    }
}
