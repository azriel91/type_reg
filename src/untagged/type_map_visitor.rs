use std::{
    fmt::{self, Debug},
    hash::Hash,
};

use serde_tagged::de::BoxFnSeed;

use crate::{
    common::{UnknownEntriesNone, UnknownEntriesSome},
    untagged::{DataTypeWrapper, TypeMap, TypeReg},
};

/// A visitor that can be used to deserialize a map of untagged values.
///
/// This visitor handles an externally tagged value, which is represented by a
/// map containing a single entry, where the key is the tag and the value is the
/// value that should be deserialized. Thus it will return an error if the
/// visited type is not a map.
///
/// The [`TypeReg`] provided to this visitor provides a [`DeserializeSeed`]
/// implementation depending on the tag, which then determines how the value is
/// going to be deserialized.
///
/// [`DeserializeSeed`]: serde::de::DeserializeSeed
pub struct TypeMapVisitor<'r, K, BoxDT, UnknownEntriesFn>
where
    K: Clone + Debug + Eq + Hash,
{
    type_reg: &'r TypeReg<K, BoxDT>,
    /// Function to deserialize an arbitrary value.
    fn_seed: UnknownEntriesFn,
}

impl<'r, K, BoxDT> TypeMapVisitor<'r, K, BoxDT, UnknownEntriesNone>
where
    K: Clone + Debug + Eq + Hash,
{
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<K, BoxDT>) -> Self {
        TypeMapVisitor {
            type_reg,
            fn_seed: UnknownEntriesNone,
        }
    }
}

impl<'r, K, BoxDT, ValueT> TypeMapVisitor<'r, K, BoxDT, BoxFnSeed<ValueT>>
where
    K: Clone + Debug + Eq + Hash,
    ValueT: Clone + Debug + Eq,
{
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<K, BoxDT>, fn_seed: BoxFnSeed<ValueT>) -> Self {
        TypeMapVisitor { type_reg, fn_seed }
    }
}

impl<'r, 'de, K, BoxDT> serde::de::Visitor<'de>
    for TypeMapVisitor<'r, K, BoxDT, UnknownEntriesNone>
where
    K: Clone + Debug + Eq + Hash + serde::Deserialize<'de> + 'de + 'static,
    BoxDT: DataTypeWrapper + 'static,
{
    type Value = TypeMap<K, BoxDT>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of arbitrary data types")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut type_map = match map_access.size_hint() {
            Some(n) => TypeMap::with_capacity_typed(n),
            _ => TypeMap::new_typed(),
        };

        while let Some(key) = map_access.next_key::<K>()? {
            let value = map_access.next_value_seed(self.type_reg.deserialize_seed(&key)?)?;
            type_map.insert_raw(key, value);
        }

        Ok(type_map)
    }
}

impl<'r, 'de, K, BoxDT, ValueT> serde::de::Visitor<'de>
    for TypeMapVisitor<'r, K, BoxDT, BoxFnSeed<ValueT>>
where
    K: Clone + Debug + Eq + Hash + serde::Deserialize<'de> + 'de + 'static,
    BoxDT: DataTypeWrapper + 'static,
    ValueT: Clone + Debug + Eq,
{
    type Value = TypeMap<K, BoxDT, UnknownEntriesSome<ValueT>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of arbitrary data types")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut type_map = match map_access.size_hint() {
            Some(n) => TypeMap::with_capacity_typed(n),
            _ => TypeMap::new_typed(),
        };

        while let Some(key) = map_access.next_key::<K>()? {
            match self.type_reg.deserialize_seed_opt(&key) {
                Some(deserialize_seed) => {
                    let value = map_access.next_value_seed(deserialize_seed)?;
                    type_map.insert_raw(key, value);
                }
                None => {
                    let value = map_access.next_value_seed(&self.fn_seed)?;
                    type_map.insert_unknown_entry(key, value);
                }
            }
        }

        Ok(type_map)
    }
}
