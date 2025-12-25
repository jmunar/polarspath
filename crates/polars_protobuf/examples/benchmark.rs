/*
This example benchmarks the direct access to the protobuf fields vs the use of the
`polars_protobuf` library.
*/

use polars_arrow::array::{Array, Int64Array, ListArray, StructArray, Utf8Array};
use polars_arrow::datatypes::{ArrowDataType, Field as ArrowField};
use polars_arrow::offset::OffsetsBuffer;
use polars_core::prelude::*;
use polars_protobuf::get_value;
use prost::Message;

#[derive(polars_structpath::StructPath, Clone, PartialEq, Message)]
struct SampleSubmessage {
    #[prost(string, tag = "1")]
    f_string: String,
    #[prost(int64, tag = "2")]
    f_integer: i64,
}

#[derive(polars_structpath::StructPath, Clone, PartialEq, Message)]
struct SampleMessage {
    #[prost(string, tag = "1")]
    f_string: String,
    #[prost(int64, tag = "2")]
    f_integer: i64,
    #[prost(double, tag = "3")]
    f_double: f64,
    #[prost(bool, tag = "4")]
    f_boolean: bool,

    #[prost(int64, optional, tag = "5")]
    f_integer_optional: Option<i64>,
    #[prost(string, optional, tag = "6")]
    f_string_optional: Option<String>,

    #[prost(int64, repeated, tag = "7")]
    f_integer_repeated: Vec<i64>,
    #[prost(string, repeated, tag = "8")]
    f_string_repeated: Vec<String>,

    #[prost(message, tag = "9")]
    #[type_hint("struct")]
    f_submessage: Option<SampleSubmessage>,

    #[prost(message, repeated, tag = "10")]
    #[type_hint("struct")]
    f_submessage_repeated: Vec<SampleSubmessage>,
}

impl SampleMessage {
    fn gen_seq(len: usize) -> ChunkedArray<BinaryType> {
        let samples: Vec<Vec<u8>> = (0..len)
            .map(|i| {
                let message = SampleMessage {
                    f_string: format!("f_string{}", i),
                    f_integer: i as i64,
                    f_double: i as f64,
                    f_boolean: i % 2 == 0,
                    f_integer_optional: if i % 2 == 0 { Some(i as i64) } else { None },
                    f_string_optional: if i % 2 == 0 {
                        Some(format!("f_string{}", i))
                    } else {
                        None
                    },
                    f_integer_repeated: (0..(i % 4)).map(|j| j as i64).collect(),
                    f_string_repeated: (0..(i % 4)).map(|j| format!("f_string{}", j)).collect(),
                    f_submessage: Some(SampleSubmessage {
                        f_string: format!("f_string{}", i),
                        f_integer: i as i64,
                    }),
                    f_submessage_repeated: (0..(i % 4))
                        .map(|j| SampleSubmessage {
                            f_string: format!("f_string{}", j),
                            f_integer: j as i64,
                        })
                        .collect(),
                };
                message.encode_to_vec()
            })
            .collect();
        ChunkedArray::from_iter(samples)
    }
}

fn print_time(label: &str, t0: std::time::Instant) {
    let t1 = std::time::Instant::now();
    println!("    {:<42} {:>10.4} s", label, (t1 - t0).as_secs_f64());
}

fn benchmark_prost_decode(samples: &ChunkedArray<BinaryType>) {
    println!("Prost decode time");
    let t0 = std::time::Instant::now();
    samples.into_iter().for_each(|bytes| {
        let _ = SampleMessage::decode(bytes.unwrap()).unwrap();
    });
    print_time("Time taken:", t0);
}

fn benchmark_f_string(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_string)
        })
        .collect::<ChunkedArray<StringType>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single = get_value::<SampleMessage>(&samples, "f_string", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded = get_value::<SampleMessage>(&samples, "f_string", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_integer(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_integer)
        })
        .collect::<ChunkedArray<Int64Type>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single = get_value::<SampleMessage>(&samples, "f_integer", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded = get_value::<SampleMessage>(&samples, "f_integer", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_double(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_double");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_double)
        })
        .collect::<ChunkedArray<Float64Type>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single = get_value::<SampleMessage>(&samples, "f_double", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded = get_value::<SampleMessage>(&samples, "f_double", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_boolean(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_boolean");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            Some(message.f_boolean)
        })
        .collect::<ChunkedArray<BooleanType>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single = get_value::<SampleMessage>(&samples, "f_boolean", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded = get_value::<SampleMessage>(&samples, "f_boolean", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_integer_optional(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer_optional");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            message.f_integer_optional
        })
        .collect::<ChunkedArray<Int64Type>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            match message.f_integer_optional {
                Some(value) => AnyValue::Int64(value),
                None => AnyValue::Null,
            }
        })
        .collect::<Vec<AnyValue>>();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single =
        get_value::<SampleMessage>(&samples, "f_integer_optional", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded =
        get_value::<SampleMessage>(&samples, "f_integer_optional", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_direct_any_value, result_direct);
    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_string_optional(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string_optional");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            message.f_string_optional
        })
        .collect::<ChunkedArray<StringType>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let string_values: Vec<Option<String>> = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            message.f_string_optional
        })
        .collect();

    let any_values: Vec<AnyValue> = string_values
        .iter()
        .map(|opt_str| match opt_str {
            Some(value) => AnyValue::String(value),
            None => AnyValue::Null,
        })
        .collect();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single =
        get_value::<SampleMessage>(&samples, "f_string_optional", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded =
        get_value::<SampleMessage>(&samples, "f_string_optional", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_direct_any_value, result_direct);
    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_integer_repeated(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_integer_repeated");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            let vec_values: ChunkedArray<Int64Type> =
                message.f_integer_repeated.into_iter().map(Some).collect();
            Some(vec_values.into_series())
        })
        .collect::<ChunkedArray<ListType>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            // Create ChunkedArray directly from Vec<i64> without intermediate Series
            let inner_ca: ChunkedArray<Int64Type> =
                message.f_integer_repeated.into_iter().map(Some).collect();
            AnyValue::List(inner_ca.into_series())
        })
        .collect::<Vec<AnyValue>>();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single =
        get_value::<SampleMessage>(&samples, "f_integer_repeated", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded =
        get_value::<SampleMessage>(&samples, "f_integer_repeated", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_direct_any_value, result_direct);
    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_string_repeated(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_string_repeated");

    let t0 = std::time::Instant::now();
    let result_direct = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            let vec_values: ChunkedArray<StringType> =
                message.f_string_repeated.into_iter().map(Some).collect();
            Some(vec_values.into_series())
        })
        .collect::<ChunkedArray<ListType>>()
        .into_series();
    print_time("Time taken (direct):", t0);

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            // Create ChunkedArray directly from Vec<String> without intermediate Series
            let inner_ca: ChunkedArray<StringType> =
                message.f_string_repeated.into_iter().map(Some).collect();
            AnyValue::List(inner_ca.into_series())
        })
        .collect::<Vec<AnyValue>>();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single =
        get_value::<SampleMessage>(&samples, "f_string_repeated", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded =
        get_value::<SampleMessage>(&samples, "f_string_repeated", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_direct_any_value, result_direct);
    assert_eq!(result_path_single, result_direct);
    assert_eq!(result_path_threaded, result_direct);
}

fn benchmark_f_submessage(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_submessage");

    // Optimized version using AnyValue to avoid intermediate allocations
    let t0 = std::time::Instant::now();

    // Define field definitions once outside the loop since they're constant
    let field_defs = vec![
        Field::new("f_string".into(), DataType::String),
        Field::new("f_integer".into(), DataType::Int64),
    ];

    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            match message.f_submessage {
                Some(ref value) => {
                    // Create a vector of AnyValues for the struct fields
                    let field_values = vec![
                        AnyValue::StringOwned(value.f_string.clone().into()),
                        AnyValue::Int64(value.f_integer),
                    ];
                    // Clone the field definitions (much cheaper than recreating)
                    AnyValue::StructOwned(Box::new((field_values, field_defs.clone())))
                }
                None => AnyValue::Null,
            }
        })
        .collect::<Vec<AnyValue>>();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single = get_value::<SampleMessage>(&samples, "f_submessage", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded = get_value::<SampleMessage>(&samples, "f_submessage", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_path_single, result_direct_any_value);
    assert_eq!(result_path_threaded, result_direct_any_value);
}

pub trait BufferAppendable {
    type ElementType;

    fn data_type(&self) -> &ArrowDataType;
    fn validity(&self) -> &Vec<bool>;
    fn new(nrows: usize) -> Self;
    fn push(&mut self, value: impl Into<Option<Self::ElementType>>);
    fn to_arrow(self) -> PolarsResult<Box<dyn Array>>;
}

struct StringBuffer {
    values: Vec<u8>,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl BufferAppendable for StringBuffer {
    type ElementType = String;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        let mut offsets = Vec::with_capacity(nrows + 1);
        offsets.push(0i32);
        Self {
            values: Vec::with_capacity(nrows),
            offsets,
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::Utf8,
        }
    }

    fn push(&mut self, value: impl Into<Option<Self::ElementType>>) {
        match value.into() {
            Some(value) => {
                self.values.extend_from_slice(value.as_bytes());
                self._validity.push(true);
            }
            None => {
                self.values.push(0);
                self._validity.push(false);
            }
        };
        self.offsets.push(self.values.len() as i32);
    }

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        let array = Utf8Array::<i32>::try_new(
            data_type,
            offsets,
            self.values.into(),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

struct I64Buffer {
    values: Vec<i64>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl BufferAppendable for I64Buffer {
    type ElementType = i64;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        Self {
            values: Vec::with_capacity(nrows),
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::Int64,
        }
    }

    fn push(&mut self, value: impl Into<Option<Self::ElementType>>) {
        match value.into() {
            Some(value) => {
                self.values.push(value);
                self._validity.push(true);
            }
            None => {
                self.values.push(0);
                self._validity.push(false);
            }
        }
    }

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let array = Int64Array::try_new(
            data_type,
            self.values.into(),
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

struct SampleSubmessageBufferInner {
    f_string: StringBuffer,
    f_integer: I64Buffer,
}

struct SampleSubmessageBuffer {
    values: SampleSubmessageBufferInner,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl BufferAppendable for SampleSubmessageBuffer {
    type ElementType = SampleSubmessage;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        let buffers = SampleSubmessageBufferInner {
            f_string: StringBuffer::new(nrows),
            f_integer: I64Buffer::new(nrows),
        };
        Self {
            values: buffers,
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::Struct(vec![
                ArrowField::new("f_string".into(), ArrowDataType::Utf8, true),
                ArrowField::new("f_integer".into(), ArrowDataType::Int64, true),
            ]),
        }
    }

    fn push(&mut self, value: impl Into<Option<Self::ElementType>>) {
        match value.into() {
            Some(value) => {
                self.values.f_string.push(value.f_string);
                self.values.f_integer.push(value.f_integer);
                self._validity.push(true);
            }
            None => {
                self.values.f_string.push(None);
                self.values.f_integer.push(None);
                self._validity.push(false);
            }
        }
    }

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let string_array = self.values.f_string.to_arrow()?;
        let integer_array = self.values.f_integer.to_arrow()?;

        let array = StructArray::try_new(
            data_type,
            self._validity.len(),
            vec![string_array, integer_array],
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

struct BufferVec<T: BufferAppendable> {
    subbuffer: T,
    offsets: Vec<i32>,
    _validity: Vec<bool>,
    _data_type: ArrowDataType,
}

impl<T: BufferAppendable> BufferAppendable for BufferVec<T> {
    type ElementType = Vec<T::ElementType>;

    fn data_type(&self) -> &ArrowDataType {
        &self._data_type
    }

    fn validity(&self) -> &Vec<bool> {
        &self._validity
    }

    fn new(nrows: usize) -> Self {
        let subbuffer = T::new(nrows);
        let mut offsets = Vec::with_capacity(nrows + 1);
        offsets.push(0i32);

        let list_field = ArrowField::new("item".into(), subbuffer.data_type().clone(), true);
        Self {
            subbuffer,
            offsets,
            _validity: Vec::with_capacity(nrows),
            _data_type: ArrowDataType::List(Box::new(list_field)),
        }
    }

    fn push(&mut self, value: impl Into<Option<Self::ElementType>>) {
        match value.into() {
            Some(value) => {
                for element in value {
                    self.subbuffer.push(element);
                }
                self._validity.push(true);
            }
            None => {
                self._validity.push(false);
            }
        }
        self.offsets.push(self.subbuffer.validity().len() as i32);
    }

    fn to_arrow(self) -> PolarsResult<Box<dyn Array>> {
        let data_type = self.data_type().clone();
        let subarray = self.subbuffer.to_arrow()?;
        let offsets = OffsetsBuffer::try_from(self.offsets)?;
        let array = ListArray::try_new(
            data_type,
            offsets,
            subarray,
            Some(polars_arrow::bitmap::Bitmap::from_iter(self._validity)),
        )?;
        Ok(Box::new(array) as Box<dyn Array>)
    }
}

fn arrow_native_construction(samples: &ChunkedArray<BinaryType>) -> Series {
    let mut list_buffer = BufferVec::<SampleSubmessageBuffer>::new(samples.len());

    // First pass: collect all data directly into buffers and compute offsets
    for bytes in samples.into_iter() {
        let message = SampleMessage::decode(bytes.unwrap()).unwrap();
        list_buffer.push(message.f_submessage_repeated);
    }

    let array_ref = list_buffer.to_arrow().unwrap();
    let series = Series::from_arrow("my_list".into(), array_ref).unwrap();
    series
}

fn benchmark_f_submessage_repeated(samples: &ChunkedArray<BinaryType>) {
    println!("Extracting f_submessage_repeated");

    let t0 = std::time::Instant::now();
    let any_values = samples
        .into_iter()
        .map(|bytes| {
            let message = SampleMessage::decode(bytes.unwrap()).unwrap();
            // Create a list of AnyValue::StructOwned for each submessage
            let struct_values: Vec<AnyValue> = message
                .f_submessage_repeated
                .into_iter()
                .map(|submessage| {
                    let field_values = vec![
                        AnyValue::StringOwned(submessage.f_string.into()),
                        AnyValue::Int64(submessage.f_integer),
                    ];
                    let field_defs = vec![
                        Field::new("f_string".into(), DataType::String),
                        Field::new("f_integer".into(), DataType::Int64),
                    ];
                    AnyValue::StructOwned(Box::new((field_values, field_defs)))
                })
                .collect();

            // Convert the Vec<AnyValue> to a Series and wrap in AnyValue::List
            let inner_series = Series::from_any_values("".into(), &struct_values, true).unwrap();
            AnyValue::List(inner_series)
        })
        .collect::<Vec<AnyValue>>();

    let result_direct_any_value = Series::from_any_values("".into(), &any_values, true).unwrap();
    print_time("Time taken (any value):", t0);

    // Pre-allocate vectors for better memory locality
    let t0 = std::time::Instant::now();
    let result_direct_arrow = arrow_native_construction(samples);
    print_time("Time taken (arrow native construction):", t0);

    let t0 = std::time::Instant::now();
    let result_path_single =
        get_value::<SampleMessage>(&samples, "f_submessage_repeated", false).unwrap();
    print_time("Time taken (structpath single-threaded):", t0);

    let t0 = std::time::Instant::now();
    let result_path_threaded =
        get_value::<SampleMessage>(&samples, "f_submessage_repeated", true).unwrap();
    print_time("Time taken (structpath multi-threaded):", t0);

    assert_eq!(result_direct_arrow, result_direct_any_value);
    assert_eq!(result_path_single, result_direct_any_value);
    assert_eq!(result_path_threaded, result_direct_any_value);
}

fn main() {
    let samples = SampleMessage::gen_seq(100000);
    benchmark_prost_decode(&samples);
    benchmark_f_string(&samples);
    benchmark_f_integer(&samples);
    benchmark_f_double(&samples);
    benchmark_f_boolean(&samples);
    benchmark_f_integer_optional(&samples);
    benchmark_f_string_optional(&samples);
    benchmark_f_integer_repeated(&samples);
    benchmark_f_string_repeated(&samples);
    benchmark_f_submessage(&samples);
    benchmark_f_submessage_repeated(&samples);
}
