use std::{fmt, hash::Hash};

use crate::{
    common::UnknownEntries,
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
pub struct TypeMapVisitor<'r, K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + fmt::Debug,
{
    type_reg: &'r TypeReg<K, BoxDT, UnknownEntriesT>,
}

impl<'r, K, BoxDT, UnknownEntriesT> TypeMapVisitor<'r, K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + fmt::Debug,
{
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<K, BoxDT, UnknownEntriesT>) -> Self {
        TypeMapVisitor { type_reg }
    }
}

impl<'r: 'de, 'de, K, BoxDT, UnknownEntriesT> serde::de::Visitor<'de>
    for TypeMapVisitor<'r, K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + fmt::Debug + serde::Deserialize<'de> + 'de,
    BoxDT: DataTypeWrapper + 'static,
    UnknownEntriesT: UnknownEntries,
{
    type Value = TypeMap<K, BoxDT, UnknownEntriesT>;

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
