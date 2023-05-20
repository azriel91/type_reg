/// `From` trait for box data types.
///
/// # Type Parameters
///
/// * `T`: The data type to wrap.
///
///     - `BoxDt` provides no additional trait constraints.
///     - `BoxDtDisplay` provides the additional `Display` constraint.
///
/// # Design
///
/// We cannot use `std::convert::From` because the `impl<T> From<T>` conflicts
/// when the `BoxDT` type is itself a `DataType`.
pub trait FromDataType<T> {
    /// Wraps the given `DataType` with Self.
    fn from(t: T) -> Self;
}
