use std::{
    fmt::{self, Debug},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use serde_tagged::de::{BoxFnSeed, SeedFactory};

use crate::{
    common::{UnknownEntriesNone, UnknownEntriesSome},
    untagged::{
        BoxDt, DataType, DataTypeWrapper, FromDataType, TypeMap, TypeMapVisitor,
        UnknownEntriesSomeFnSeed,
    },
};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map from a given key to logic to deserialize a type.
pub struct TypeReg<K, BoxDT = BoxDt, UnknownEntriesT = UnknownEntriesNone>
where
    K: Eq + Hash + Debug,
{
    fn_seeds: Map<K, BoxFnSeed<BoxDT>>,
    unknown_entries: UnknownEntriesT,
}

impl<K> TypeReg<K, BoxDt, UnknownEntriesNone>
where
    K: Eq + Hash + Debug,
{
    // Creates an empty `TypeReg`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeReg;
    /// let mut type_reg = TypeReg::<&'static str>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            fn_seeds: Map::new(),
            unknown_entries: UnknownEntriesNone,
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
    /// use type_reg::untagged::TypeReg;
    /// let type_reg = TypeReg::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            fn_seeds: Map::with_capacity(capacity),
            unknown_entries: UnknownEntriesNone,
        }
    }
}

impl<K, BoxDT> TypeReg<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + Debug + 'static,
    BoxDT: DataTypeWrapper + 'static,
{
    // Creates an empty `TypeReg`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeReg;
    /// let mut type_reg = TypeReg::<&'static str>::new();
    /// ```
    pub fn new_typed() -> Self {
        Self {
            fn_seeds: Map::new(),
            unknown_entries: UnknownEntriesNone,
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
    /// use type_reg::untagged::TypeReg;
    /// let type_reg = TypeReg::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity_typed(capacity: usize) -> Self {
        Self {
            fn_seeds: Map::with_capacity(capacity),
            unknown_entries: UnknownEntriesNone,
        }
    }

    pub fn with_unknown_entries<ValueT>(self) -> TypeReg<K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>
    where
        ValueT: serde::de::DeserializeOwned + 'static,
    {
        let Self {
            fn_seeds,
            unknown_entries: UnknownEntriesNone,
        } = self;

        TypeReg {
            fn_seeds,
            unknown_entries: UnknownEntriesSomeFnSeed::new(BoxFnSeed::new(
                Self::deserialize_value::<ValueT>,
            )),
        }
    }

    fn deserialize_value<ValueT>(
        deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<ValueT, erased_serde::Error>
    where
        ValueT: serde::de::DeserializeOwned + 'static,
    {
        ValueT::deserialize(deserializer)
    }
}

impl<K, BoxDT, UnknownEntriesT> TypeReg<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + Debug + 'static,
    BoxDT: DataTypeWrapper + 'static,
    UnknownEntriesT: 'static,
{
    /// Registers a type in this type registry.
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type, or map of types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeReg;
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str("one: 1");
    ///
    /// let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    /// let data_u32 = data_u32.downcast_ref::<u32>().copied();
    ///
    /// println!("{data_u32:?}"); // prints "1"
    /// ```
    pub fn register<R>(&mut self, key: K)
    where
        R: serde::de::DeserializeOwned + DataType + 'static,
        BoxDT: FromDataType<R>,
    {
        self.fn_seeds
            .insert(key, BoxFnSeed::new(Self::deserialize::<R>));
    }

    fn deserialize<R>(
        deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<BoxDT, erased_serde::Error>
    where
        R: serde::de::DeserializeOwned + DataType + 'static,
        BoxDT: FromDataType<R>,
    {
        Ok(<BoxDT as FromDataType<R>>::from(R::deserialize(
            deserializer,
        )?))
    }
}

impl<K, BoxDT> TypeReg<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + Debug,
    BoxDT: DataTypeWrapper + 'static,
{
    /// Deserializes a map of arbitrary values into a [`TypeMap`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMap, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    /// type_reg.register::<u64>(String::from("two"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str(
    ///     "---\n\
    ///     one: 1\n\
    ///     two: 2\n\
    ///     ",
    /// );
    ///
    /// let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
    /// let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    /// let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    ///
    /// println!("{data_u32}, {data_u64}"); // prints "1, 2"
    /// ```
    pub fn deserialize_map<'de, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMap<K, BoxDT, UnknownEntriesNone>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapVisitor::<K, BoxDT, UnknownEntriesNone>::new(self);
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
    /// use type_reg::untagged::TypeReg;
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str("one: 1");
    ///
    /// let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    /// let data_u32 = data_u32.downcast_ref::<u32>().copied();
    ///
    /// println!("{data_u32:?}"); // prints "1"
    /// ```
    pub fn deserialize_single<'de, D, E>(&self, deserializer: D) -> Result<BoxDT, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        serde_tagged::de::external::deserialize(deserializer, self)
    }

    pub(crate) fn deserialize_seed<E>(&self, type_key: &K) -> Result<&BoxFnSeed<BoxDT>, E>
    where
        E: serde::de::Error,
    {
        self.fn_seeds.get(type_key).ok_or_else(|| {
            use std::fmt::Write;
            let mut message = String::with_capacity(256);
            write!(
                message,
                "Type key `{type_key:?}` not registered in type registry."
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

impl<K, BoxDT, ValueT> TypeReg<K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>
where
    K: Clone + Eq + Hash + Debug,
    BoxDT: DataTypeWrapper + 'static,
    ValueT: Clone + Debug + Eq,
{
    /// Deserializes a map of arbitrary values into a [`TypeMap`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMap, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    /// type_reg.register::<u64>(String::from("two"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str(
    ///     "---\n\
    ///     one: 1\n\
    ///     two: 2\n\
    ///     ",
    /// );
    ///
    /// let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
    /// let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    /// let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    ///
    /// println!("{data_u32}, {data_u64}"); // prints "1, 2"
    /// ```
    pub fn deserialize_map<'de, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMap<K, BoxDT, UnknownEntriesSome<ValueT>>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapVisitor::<K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>::new(self);
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
    /// use type_reg::untagged::TypeReg;
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str("one: 1");
    ///
    /// let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    /// let data_u32 = data_u32.downcast_ref::<u32>().copied();
    ///
    /// println!("{data_u32:?}"); // prints "1"
    /// ```
    pub fn deserialize_single<'de, D, E>(&self, deserializer: D) -> Result<BoxDT, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        serde_tagged::de::external::deserialize(deserializer, self)
    }

    pub(crate) fn deserialize_seed_opt(&self, type_key: &K) -> Option<&BoxFnSeed<BoxDT>> {
        self.fn_seeds.get(type_key)
    }

    pub(crate) fn value_deserialize_seed(&self) -> &BoxFnSeed<ValueT> {
        self.unknown_entries.fn_seed()
    }
}

impl<K, BoxDT> Default for TypeReg<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + Debug,
{
    fn default() -> Self {
        Self {
            fn_seeds: Map::default(),
            unknown_entries: UnknownEntriesNone,
        }
    }
}

impl<K, BoxDT, UnknownEntriesT> Debug for TypeReg<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + Debug,
{
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

impl<K, BoxDT, UnknownEntriesT> Deref for TypeReg<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + Debug,
{
    type Target = Map<K, BoxFnSeed<BoxDT>>;

    fn deref(&self) -> &Self::Target {
        &self.fn_seeds
    }
}

impl<K, BoxDT, UnknownEntriesT> DerefMut for TypeReg<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fn_seeds
    }
}

// Used by [`serde_tagged`] to select which [`DeserializeSeed`] function to use.
impl<'r, 'de, K, BoxDT> SeedFactory<'de, K> for &'r TypeReg<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + Debug + 'de,
    BoxDT: DataTypeWrapper + 'static,
{
    type Seed = &'r BoxFnSeed<BoxDT>;
    type Value = BoxDT;

    fn seed<E>(self, type_key: K) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.deserialize_seed(&type_key)
    }
}

impl<'r, 'de, K, BoxDT, ValueT> SeedFactory<'de, K>
    for &'r TypeReg<K, BoxDT, UnknownEntriesSomeFnSeed<ValueT>>
where
    K: Clone + Eq + Hash + Debug + 'de,
    BoxDT: DataTypeWrapper + 'static,
    ValueT: Clone + Debug + Eq,
{
    type Seed = &'r BoxFnSeed<BoxDT>;
    type Value = BoxDT;

    fn seed<E>(self, type_key: K) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.deserialize_seed_opt(&type_key).ok_or_else(|| {
            use std::fmt::Write;
            let mut message = String::with_capacity(256);
            write!(
                message,
                "Type key `{type_key:?}` not registered in type registry."
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

#[cfg(test)]
mod tests {
    use crate::untagged::{BoxDataTypeDowncast, TypeMap, TypeReg};
    use serde::{Deserialize, Serialize};

    #[test]
    fn deserialize_single() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));

        let deserializer = serde_yaml::Deserializer::from_str("one: 1");
        let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
        let data_u32 = BoxDataTypeDowncast::<u32>::downcast_ref(&data_u32).copied();

        assert_eq!(Some(1), data_u32);
    }

    #[test]
    fn deserialize_map() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<u64>(String::from("two"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: 3\n\
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
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let deserializer = serde_yaml::Deserializer::from_str("two: 2");
        if let Err(error) = type_reg.deserialize_single(deserializer) {
            assert_eq!(
                r#"Type key `"two"` not registered in type registry.
Available types are:

- "one"
- "three"

"#,
                format!("{error}")
            );
        } else {
            panic!("Expected `deserialize_single` to return error.");
        }
    }

    #[test]
    fn deserialize_map_with_unknown_entries_yaml() {
        let mut type_reg = TypeReg::<String>::new().with_unknown_entries::<serde_yaml::Value>();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: 3\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map = type_reg.deserialize_map(deserializer).unwrap();

        let data_u32 = type_map.get::<u32, _>("one").copied();
        let data_u64 = type_map.get_unknown_entry("two").cloned();
        let data_a = type_map.get::<A, _>("three").copied();

        assert_eq!(Some(1u32), data_u32);
        assert_eq!(Some(A(3)), data_a);

        assert_eq!(
            data_u64,
            Some(serde_yaml::Value::Number(serde_yaml::Number::from(2u64)))
        );
        assert_eq!(1, type_map.unknown_entries().len());
    }

    #[test]
    fn deserialize_map_with_unknown_entries_json() {
        let mut type_reg = TypeReg::<String>::new().with_unknown_entries::<serde_json::Value>();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let serialized = r#"
            {
                "one": 1,
                "two": 2,
                "three": 3
            }
        "#;

        let mut deserializer = serde_json::Deserializer::from_str(serialized);
        let type_map = type_reg.deserialize_map(&mut deserializer).unwrap();

        let data_u32 = type_map.get::<u32, _>("one").copied();
        let data_u64 = type_map.get_unknown_entry("two").cloned();
        let data_a = type_map.get::<A, _>("three").copied();

        assert_eq!(Some(1u32), data_u32);
        assert_eq!(Some(A(3)), data_a);

        assert_eq!(
            data_u64,
            Some(serde_json::Value::Number(serde_json::Number::from(2u64)))
        );
        assert_eq!(1, type_map.unknown_entries().len());
    }

    #[test]
    fn with_capacity() {
        let type_reg = TypeReg::<String>::default();
        assert_eq!(0, type_reg.capacity());

        let type_reg = TypeReg::<String>::with_capacity(5);
        assert!(type_reg.capacity() >= 5);
    }

    #[test]
    fn debug() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<A>("one");

        assert_eq!(r#"{"one": ".."}"#, format!("{type_reg:?}"));
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);
}
