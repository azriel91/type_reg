use std::fmt;

use serde_tagged::de::BoxFnSeed;

use crate::common::UnknownEntries;

/// Indicates unknown entries are not stored in a given `TypeMap`.
pub struct UnknownEntriesSomeFnSeed<ValueT> {
    /// Function to deserialize an arbitrary value.
    fn_seed: BoxFnSeed<ValueT>,
    /// Function to deserialize an arbitrary optional value.
    fn_opt_seed: BoxFnSeed<Option<ValueT>>,
}

impl<ValueT> UnknownEntriesSomeFnSeed<ValueT> {
    /// Returns a new `UnknownEntriesSomeFnSeed` with a function to deserialize
    /// an arbitrary value.
    pub fn new(fn_seed: BoxFnSeed<ValueT>, fn_opt_seed: BoxFnSeed<Option<ValueT>>) -> Self {
        Self {
            fn_seed,
            fn_opt_seed,
        }
    }

    /// Returns the `fn_seed` for deserializing an arbitrary value.
    pub fn fn_seed(&self) -> &BoxFnSeed<ValueT> {
        &self.fn_seed
    }

    /// Returns the `fn_opt_seed` for deserializing an arbitrary optional value.
    pub fn fn_opt_seed(&self) -> &BoxFnSeed<Option<ValueT>> {
        &self.fn_opt_seed
    }
}

impl<ValueT> fmt::Debug for UnknownEntriesSomeFnSeed<ValueT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UnknownEntriesSomeFnSeed")
            .field("fn_seed", &std::any::type_name::<BoxFnSeed<ValueT>>())
            .field(
                "fn_opt_seed",
                &std::any::type_name::<BoxFnSeed<Option<ValueT>>>(),
            )
            .finish()
    }
}

impl<#[cfg(not(feature = "debug"))] ValueT, #[cfg(feature = "debug")] ValueT: std::fmt::Debug>
    UnknownEntries for UnknownEntriesSomeFnSeed<ValueT>
where
    ValueT: Clone + Eq,
{
    type ValueT = ValueT;
}
