use std::{fmt, hash::Hash};

use crate::{
    common::{UnknownEntriesNone, UnknownEntriesSome},
    untagged::{DataTypeWrapper, TypeMapOpt, TypeReg, UnknownEntriesSomeFnSeed},
};

/// A visitor that can be used to deserialize a map of untagged optional values.
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
pub struct TypeMapOptVisitor<'r, K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + fmt::Debug,
{
    type_reg: &'r TypeReg<K, BoxDT, UnknownEntriesT>,
}

impl<'r, K, BoxDT> TypeMapOptVisitor<'r, K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + fmt::Debug,
{
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<K, BoxDT, UnknownEntriesNone>) -> Self {
        TypeMapOptVisitor { type_reg }
    }
}

impl<
    'r,
    K,
    BoxDT,
    #[cfg(not(feature = "debug"))] ValueT,
    #[cfg(feature = "debug")] ValueT: std::fmt::Debug,
> TypeMapOptVisitor<'r, K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>
where
    K: Eq + Hash + fmt::Debug,
    ValueT: Clone + Eq,
{
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>) -> Self {
        TypeMapOptVisitor { type_reg }
    }
}

impl<'r: 'de, 'de, K, BoxDT> serde::de::Visitor<'de>
    for TypeMapOptVisitor<'r, K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + fmt::Debug + serde::Deserialize<'de> + 'de,
    BoxDT: DataTypeWrapper + 'static,
{
    type Value = TypeMapOpt<K, BoxDT, UnknownEntriesNone>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of arbitrary data types")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut type_map = match map_access.size_hint() {
            Some(n) => TypeMapOpt::with_capacity_typed(n),
            _ => TypeMapOpt::new_typed(),
        };

        while let Some(key) = map_access.next_key::<K>()? {
            let value = map_access.next_value_seed(self.type_reg.deserialize_opt_seed(&key)?)?;
            type_map.insert_raw(key, value);
        }

        Ok(type_map)
    }
}

impl<'r: 'de, 'de, K, BoxDT, ValueT> serde::de::Visitor<'de>
    for TypeMapOptVisitor<'r, K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>
where
    K: Clone + Eq + Hash + fmt::Debug + serde::Deserialize<'de> + 'de,
    BoxDT: DataTypeWrapper + 'static,
    ValueT: Clone + fmt::Debug + Eq,
{
    type Value = TypeMapOpt<K, BoxDT, UnknownEntriesSome<ValueT>>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of arbitrary data types")
    }

    fn visit_map<A>(self, mut map_access: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut type_map = match map_access.size_hint() {
            Some(n) => TypeMapOpt::with_capacity_typed(n),
            _ => TypeMapOpt::new_typed(),
        };

        while let Some(key) = map_access.next_key::<K>()? {
            match self.type_reg.deserialize_opt_seed_opt(&key) {
                Some(deserialize_opt_seed) => {
                    let value = map_access.next_value_seed(deserialize_opt_seed)?;
                    type_map.insert_raw(key, value);
                }
                None => {
                    let value_deserialize_opt_seed = self.type_reg.value_deserialize_opt_seed();
                    let value = map_access.next_value_seed(value_deserialize_opt_seed)?;
                    type_map.insert_unknown_entry(key, value);
                }
            }
        }

        Ok(type_map)
    }
}
