use std::marker::PhantomData;

/// Indicates unknown entries are not stored in a given `TypeMap`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownEntriesNone;

/// Indicates unknown entries are not stored in a given `TypeMap`.
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use super::{UnknownEntriesNone, UnknownEntriesSome};

    #[test]
    fn clone() {
        let _unknown_entries_none = Clone::clone(&UnknownEntriesNone);
        let _unknown_entries_some = Clone::clone(&UnknownEntriesSome::<()>(PhantomData));
    }

    #[test]
    fn debug() {
        assert_eq!("UnknownEntriesNone", format!("{UnknownEntriesNone:?}"));
        assert_eq!(
            "UnknownEntriesSome(PhantomData<()>)",
            format!("{:?}", UnknownEntriesSome::<()>(PhantomData))
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(UnknownEntriesNone, UnknownEntriesNone);
        assert_eq!(
            UnknownEntriesSome::<()>(PhantomData),
            UnknownEntriesSome::<()>(PhantomData)
        );
    }
}
