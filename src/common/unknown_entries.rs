use std::marker::PhantomData;

/// Indicates unknown entries are not stored in a given `TypeMap`.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct UnknownEntriesNone;

/// Indicates unknown entries are not stored in a given `TypeMap`.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, PartialEq, Eq)]
pub struct UnknownEntriesSome<ValueT>(PhantomData<ValueT>);

/// Associates an `UnknownEntries` type parameter with the deserialization
/// format's generic value type.
pub trait UnknownEntries {
    #[cfg(not(feature = "debug"))]
    type ValueT: Clone + PartialEq + Eq;
    #[cfg(feature = "debug")]
    type ValueT: Clone + std::fmt::Debug + PartialEq + Eq;
}

impl UnknownEntries for UnknownEntriesNone {
    type ValueT = ();
}

#[cfg(not(feature = "debug"))]
impl<ValueT> UnknownEntries for UnknownEntriesSome<ValueT>
where
    ValueT: Clone + Eq,
{
    type ValueT = ValueT;
}

#[cfg(feature = "debug")]
impl<ValueT> UnknownEntries for UnknownEntriesSome<ValueT>
where
    ValueT: Clone + std::fmt::Debug + Eq,
{
    type ValueT = ValueT;
}
