/// Types that can be boxed.
///
/// # Type Parameters
///
/// * `BoxDT`: The specific box data type.
///     - `BoxDt` provides no additional trait constraints.
///     - `BoxDtDisplay` provides the additional `Display` constraint.
pub trait IntoBoxDataType<BoxDT> {
    /// Turns self into a boxed `DataType`.
    fn into(t: Self) -> BoxDT;
}
