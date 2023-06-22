use std::{fmt, hash::Hash, marker::PhantomData};

use crate::{
    common::UnknownEntries,
    tagged::{TypeMap, TypeReg},
};

/// A visitor that can be used to deserialize a map of externally tagged values.
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
pub struct TypeMapVisitor<'key, 'r, MapK, UnknownEntriesT> {
    type_reg: &'r TypeReg<'key, UnknownEntriesT>,
    marker: PhantomData<MapK>,
}

impl<'key, 'r, MapK, UnknownEntriesT> TypeMapVisitor<'key, 'r, MapK, UnknownEntriesT> {
    /// Creates a new visitor with the given [`TypeReg`].
    pub fn new(type_reg: &'r TypeReg<'key, UnknownEntriesT>) -> Self {
        TypeMapVisitor {
            type_reg,
            marker: PhantomData,
        }
    }
}

impl<'key: 'de, 'de: 'r, 'r, MapK, UnknownEntriesT> serde::de::Visitor<'de>
    for TypeMapVisitor<'key, 'r, MapK, UnknownEntriesT>
where
    MapK: Eq + Hash + fmt::Debug + serde::Deserialize<'de> + 'de,
    UnknownEntriesT: UnknownEntries,
{
    type Value = TypeMap<MapK, UnknownEntriesT>;

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

        while let Some(key) = map_access.next_key::<MapK>()? {
            let value = map_access.next_value_seed(self.type_reg)?;

            type_map.insert_raw(key, value);
        }

        Ok(type_map)
    }
}
