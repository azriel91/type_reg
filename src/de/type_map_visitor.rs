use std::{fmt, hash::Hash, marker::PhantomData};

use crate::{TypeMap, TypeReg};

/// A visitor that can be used to deserialize a map of externally tagged values.
///
/// This visitor handles an externally tagged value, which is represented by a
/// map containing a single entry, where the key is the tag and the value is the
/// value that should be deserialized. Thus it will return an error if the
/// visited type is not a map.
///
/// The [`SeedFactory`](::de::SeedFactory) provided to this visitor
/// provides a `serde::de::DeserializeSeed` implementation depending on the tag,
/// which then determines how the value is going to be deserialized.
///
/// See [`de`](::de) for more information on
/// [`SeedFactory`](::de::SeedFactory) and implementations thereof.
pub struct TypeMapVisitor<'key, 'r, MapK> {
    type_reg: &'r TypeReg<'key>,
    marker: PhantomData<MapK>,
}

impl<'key, 'r, MapK> TypeMapVisitor<'key, 'r, MapK> {
    /// Creates a new visitor with the given [`SeedFactory`](::de::SeedFactory).
    pub fn new(type_reg: &'r TypeReg<'key>) -> Self {
        TypeMapVisitor {
            type_reg,
            marker: PhantomData,
        }
    }
}

impl<'key: 'de, 'de: 'r, 'r, MapK> serde::de::Visitor<'de> for TypeMapVisitor<'key, 'r, MapK>
where
    MapK: Eq + Hash + fmt::Debug + serde::Deserialize<'de> + 'de,
{
    type Value = TypeMap<MapK>;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "a map of arbitrary data types")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut type_map = match map.size_hint() {
            Some(n) => TypeMap::with_capacity(n),
            _ => TypeMap::new(),
        };

        while let Some(key) = map.next_key::<MapK>()? {
            let value = map.next_value_seed(self.type_reg)?;

            type_map.insert(key, value);
        }

        Ok(type_map)
    }
}
