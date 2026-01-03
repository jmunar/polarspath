use polars_arrow::array::Array;
use polars_arrow::datatypes::ArrowDataType;
use polars_core::prelude::*;

/// Trait for types that can accumulate values and convert them to Arrow arrays.
///
/// `ArrowBuffer` is the core trait for building Arrow arrays incrementally. Implementations
/// accumulate values through `push()` and `push_null()`, then convert to an Arrow array via
/// `to_arrow()`.
///
/// # Type Parameters
///
/// - `Element`: The Rust type being accumulated (e.g., `i32`, `String`, `Person`)
/// - `Arrow`: The corresponding Arrow array type (e.g., `PrimitiveArray<i32>`, `Utf8Array<i32>`)
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::ArrowBuffer;
///
/// let mut buffer = i32::new_buffer(3);
/// buffer.push(1);
/// buffer.push(2);
/// buffer.push_null();
/// let array = buffer.to_arrow().unwrap();
/// ```
pub trait ArrowBuffer {
    /// The Rust element type being accumulated.
    type Element;

    /// The Arrow array type produced by this buffer.
    type Arrow: Array;

    /// Returns the Arrow data type for this buffer.
    fn data_type(&self) -> &ArrowDataType;

    /// Returns the validity bitmap for this buffer.
    fn validity(&self) -> &Vec<bool>;

    /// Creates a new buffer with capacity for `nrows` elements.
    fn new(nrows: usize) -> Self;

    /// Pushes a value into the buffer.
    ///
    /// The value is converted via `Into<Self::Element>`, allowing flexible input types.
    fn push(&mut self, value: impl Into<Self::Element>);

    /// Pushes a null value into the buffer.
    ///
    /// This marks the current position as null in the validity bitmap.
    fn push_null(&mut self);

    /// Converts the buffer into an Arrow array.
    ///
    /// Consumes the buffer and returns a `PolarsResult` containing the Arrow array.
    fn to_arrow(self) -> PolarsResult<Self::Arrow>;
}

/// Trait for types that can be converted to Arrow arrays.
///
/// Types implementing `IntoArrow` can create an `ArrowBuffer` for accumulating values.
/// This trait provides the convenience method `new_buffer()` which creates a buffer
/// ready to accept values of this type.
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::{IntoArrow, ArrowBuffer};
///
/// let mut buffer = String::new_buffer(2);
/// buffer.push("hello");
/// buffer.push("world");
/// ```
pub trait IntoArrow: Sized {
    /// The buffer type used to accumulate values of this type.
    type Buffer: ArrowBuffer;

    /// Creates a new buffer with capacity for `nrows` elements.
    ///
    /// This is a convenience method that delegates to `Buffer::new()`.
    fn new_buffer(nrows: usize) -> Self::Buffer {
        Self::Buffer::new(nrows)
    }
}

/// Trait for types that can be converted from Arrow arrays.
///
/// `FromArrow` enables bidirectional conversion, allowing you to read data back from
/// Arrow arrays into Rust types. It provides two methods:
///
/// - `from_arrow()`: Converts to `Vec<Self>`, assuming all values are non-null
/// - `from_arrow_opt()`: Converts to `Vec<Option<Self>>`, preserving null information
///
/// # Example
///
/// ```rust
/// use polars_structpath_types::{IntoArrow, FromArrow, ArrowBuffer};
///
/// // Create and populate a buffer
/// let mut buffer = i32::new_buffer(3);
/// buffer.push(1);
/// buffer.push(2);
/// buffer.push(3);
///
/// // Convert to Arrow array
/// let array = buffer.to_arrow().unwrap();
///
/// // Convert back from Arrow array
/// let values: Vec<i32> = i32::from_arrow(Box::new(array));
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
pub trait FromArrow
where
    Self: Sized,
{
    /// Converts an Arrow array to a `Vec<Self>`.
    ///
    /// Assumes all values are non-null. For nullable arrays, use `from_arrow_opt()`.
    fn from_arrow(array: Box<dyn Array>) -> Vec<Self>;

    /// Converts an Arrow array to a `Vec<Option<Self>>`.
    ///
    /// Preserves null information from the Arrow array's validity bitmap.
    fn from_arrow_opt(array: Box<dyn Array>) -> Vec<Option<Self>>;
}
