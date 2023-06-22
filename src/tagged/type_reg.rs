use std::{
    borrow::Cow,
    fmt,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::de::DeserializeSeed;
use serde_tagged::de::{BoxFnSeed, SeedFactory};

use crate::{
    common::{UnknownEntries, UnknownEntriesNone},
    tagged::{DataType, TypeMap, TypeMapVisitor},
};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map from a given key to logic to deserialize a type.
#[derive(Default)]
pub struct TypeReg<'key, UnknownEntriesT = UnknownEntriesNone> {
    fn_seeds: Map<Cow<'key, str>, BoxFnSeed<Box<dyn DataType>>>,
    marker: PhantomData<UnknownEntriesT>,
}

impl<'key> TypeReg<'key, UnknownEntriesNone> {
    // Creates an empty `TypeReg`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeReg;
    /// let mut type_reg = TypeReg::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fn_seeds: Map::new(),
            marker: PhantomData,
        }
    }

    /// Creates an empty `TypeReg` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeReg;
    /// let type_reg = TypeReg::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            fn_seeds: Map::with_capacity(capacity),
            marker: PhantomData,
        }
    }
}

impl<'key, UnknownEntriesT> TypeReg<'key, UnknownEntriesT> {
    /// Registers a type in this type registry.
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type, or map of types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeReg;
    ///
    /// let mut type_reg = TypeReg::new();
    /// type_reg.register::<u32>();
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str("u32: 1");
    ///
    /// let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    /// let data_u32 = data_u32.downcast_ref::<u32>().copied();
    ///
    /// println!("{data_u32:?}"); // prints "1"
    /// ```
    pub fn register<R>(&mut self)
    where
        R: serde::de::DeserializeOwned + DataType + 'static,
    {
        fn deserialize<R>(
            deserializer: &mut dyn erased_serde::Deserializer<'_>,
        ) -> Result<Box<dyn DataType>, erased_serde::Error>
        where
            R: serde::de::DeserializeOwned + DataType + 'static,
        {
            Ok(Box::new(R::deserialize(deserializer)?))
        }

        self.fn_seeds.insert(
            Cow::Borrowed(std::any::type_name::<R>()),
            BoxFnSeed::new(deserialize::<R>),
        );
    }

    /// Deserializes a map of arbitrary values into a [`TypeMap`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::{TypeMap, TypeReg};
    ///
    /// let mut type_reg = TypeReg::new();
    /// type_reg.register::<u32>();
    /// type_reg.register::<u64>();
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str(
    ///     "---\n\
    ///     one: { u32: 1 }\n\
    ///     two: { u64: 2 }\n\
    ///     ",
    /// );
    ///
    /// let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
    /// let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    /// let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    ///
    /// println!("{data_u32}, {data_u64}"); // prints "1, 2"
    /// ```
    pub fn deserialize_map<'de, MapK, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMap<MapK, UnknownEntriesT>, E>
    where
        MapK: Eq
            + Hash
            + fmt::Debug
            + Send
            + Sync
            + serde::Serialize
            + serde::Deserialize<'de>
            + 'static,
        UnknownEntriesT: UnknownEntries,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapVisitor::new(self);
        deserializer.deserialize_map(visitor)
    }

    /// Deserializes an arbitrary value into a [`DataType`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeReg;
    ///
    /// let mut type_reg = TypeReg::new();
    /// type_reg.register::<u32>();
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str("u32: 1");
    ///
    /// let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    /// let data_u32 = data_u32.downcast_ref::<u32>().copied();
    ///
    /// println!("{data_u32:?}"); // prints "1"
    /// ```
    pub fn deserialize_single<'de, D, E>(&'de self, deserializer: D) -> Result<Box<dyn DataType>, E>
    where
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        serde_tagged::de::external::deserialize(deserializer, self)
    }
}

impl<'key, UnknownEntriesT> fmt::Debug for TypeReg<'key, UnknownEntriesT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        // BoxFnSeed is `!Debug`, so we just use "..".
        self.fn_seeds.keys().for_each(|k| {
            debug_map.key(&k);
            debug_map.value(&"..");
        });

        debug_map.finish()
    }
}

impl<'key, UnknownEntriesT> Deref for TypeReg<'key, UnknownEntriesT> {
    type Target = Map<Cow<'key, str>, BoxFnSeed<Box<dyn DataType>>>;

    fn deref(&self) -> &Self::Target {
        &self.fn_seeds
    }
}

impl<'key, UnknownEntriesT> DerefMut for TypeReg<'key, UnknownEntriesT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fn_seeds
    }
}

// Used by [`serde_tagged`] to select which [`DeserializeSeed`] function to use.
impl<'key: 'de, 'de: 'r, 'r, UnknownEntriesT> SeedFactory<'de, Cow<'de, str>>
    for &'r TypeReg<'key, UnknownEntriesT>
{
    type Seed = &'r BoxFnSeed<Box<dyn DataType>>;
    type Value = Box<dyn DataType>;

    fn seed<E>(self, type_tag: Cow<'de, str>) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.fn_seeds.get(&type_tag).ok_or_else(|| {
            use std::fmt::Write;
            let mut message = String::with_capacity(256);
            write!(
                message,
                "Type `{type_tag:?}` not registered in type registry."
            )
            .expect("Failed to write error message");

            message.push_str("\nAvailable types are:\n\n");
            let mut message = self
                .fn_seeds
                .keys()
                .try_fold(message, |mut message, key| {
                    writeln!(message, "- {key:?}")?;
                    Result::<_, fmt::Error>::Ok(message)
                })
                .expect("Failed to write error message");
            message.push('\n');

            serde::de::Error::custom(message)
        })
    }
}

// Used when [`TypeReg`] is used as the seed to deserialize an arbitrary
// [`DataType`].
impl<'key: 'de, 'de: 'r, 'r, UnknownEntriesT> DeserializeSeed<'de>
    for &'r TypeReg<'key, UnknownEntriesT>
{
    type Value = Box<dyn DataType>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        serde_tagged::de::external::deserialize(deserializer, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::tagged::{TypeMap, TypeReg};
    use serde::{Deserialize, Serialize};

    #[test]
    fn deserialize_single() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<u32>();

        let deserializer = serde_yaml::Deserializer::from_str("u32: 1");
        let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
        let data_u32 = data_u32.downcast_ref::<u32>().copied();

        assert_eq!(Some(1), data_u32);
    }

    #[test]
    fn deserialize_map() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<u32>();
        type_reg.register::<u64>();
        type_reg.register::<A>();

        let serialized = "---\n\
            one:   { u32: 1 }\n\
            two:   { u64: 2 }\n\
            three: { 'type_reg::tagged::type_reg::tests::A': 3 }\n\
            ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();

        let data_u32 = type_map.get::<u32, _>("one").copied();
        let data_u64 = type_map.get::<u64, _>("two").copied();
        let data_a = type_map.get::<A, _>("three").copied();

        assert_eq!(Some(1u32), data_u32);
        assert_eq!(Some(2u64), data_u64);
        assert_eq!(Some(A(3)), data_a);
    }

    #[cfg(feature = "ordered")]
    #[test]
    fn deserialize_has_good_error_message() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<u32>();
        type_reg.register::<A>();

        let deserializer = serde_yaml::Deserializer::from_str("u64: 2");
        if let Err(error) = type_reg.deserialize_single(deserializer) {
            assert_eq!(
                r#"Type `"u64"` not registered in type registry.
Available types are:

- "u32"
- "type_reg::tagged::type_reg::tests::A"

"#,
                format!("{error}")
            );
        } else {
            panic!("Expected `deserialize_single` to return error.");
        }
    }

    #[test]
    fn with_capacity() {
        let type_reg = TypeReg::new();
        assert_eq!(0, type_reg.capacity());

        let type_reg = TypeReg::with_capacity(5);
        assert!(type_reg.capacity() >= 5);
    }

    #[test]
    fn debug() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<A>();

        assert_eq!(
            r#"{"type_reg::tagged::type_reg::tests::A": ".."}"#,
            format!("{type_reg:?}")
        );
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);
}
