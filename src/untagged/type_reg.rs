use std::{
    fmt::{self, Debug},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use serde::de::DeserializeOwned;
use serde_tagged::de::{BoxFnSeed, SeedFactory};

use crate::{
    common::{UnknownEntriesNone, UnknownEntriesSome},
    untagged::{
        BoxDt, DataType, DataTypeWrapper, FromDataType, TypeMap, TypeMapOpt, TypeMapOptVisitor,
        TypeMapVisitor,
    },
};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map from a given key to logic to deserialize a type.
pub struct TypeReg<K, BoxDT = BoxDt>
where
    K: Eq + Hash + Debug,
{
    fn_seeds: Map<K, BoxFnSeed<BoxDT>>,
    fn_opt_seeds: Map<K, BoxFnSeed<Option<BoxDT>>>,
}

impl<K> TypeReg<K, BoxDt>
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
            fn_opt_seeds: Map::new(),
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
            fn_opt_seeds: Map::with_capacity(capacity),
        }
    }
}

impl<K, BoxDT> TypeReg<K, BoxDT>
where
    K: Clone + Debug + Eq + Hash + 'static,
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
            fn_opt_seeds: Map::new(),
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
            fn_opt_seeds: Map::with_capacity(capacity),
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

    fn deserialize_opt_value<ValueT>(
        deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<Option<ValueT>, erased_serde::Error>
    where
        Option<ValueT>: serde::de::DeserializeOwned + 'static,
    {
        use serde::Deserialize;
        Option::<ValueT>::deserialize(deserializer)
    }

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
            .insert(key.clone(), BoxFnSeed::new(Self::deserialize::<R>));
        self.fn_opt_seeds
            .insert(key, BoxFnSeed::new(Self::deserialize_opt::<R>));
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

    fn deserialize_opt<R>(
        deserializer: &mut dyn erased_serde::Deserializer<'_>,
    ) -> Result<Option<BoxDT>, erased_serde::Error>
    where
        Option<R>: serde::de::DeserializeOwned,
        R: DataType + 'static,
        BoxDT: FromDataType<R>,
    {
        use serde::de::Deserialize;
        Ok(Option::<R>::deserialize(deserializer)?.map(<BoxDT as FromDataType<R>>::from))
    }

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
    pub fn deserialize_map<'de, D, E>(&'de self, deserializer: D) -> Result<TypeMap<K, BoxDT>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapVisitor::<K, BoxDT, UnknownEntriesNone>::new(self);
        deserializer.deserialize_map(visitor)
    }

    /// Deserializes a map of arbitrary values into a [`TypeMapOpt`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMapOpt, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    /// type_reg.register::<u64>(String::from("two"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str(
    ///     "---\n\
    ///     one: 1\n\
    ///     two: null\n\
    ///     ",
    /// );
    ///
    /// let type_map_opt: TypeMapOpt<String> = type_reg.deserialize_map_opt(deserializer).unwrap();
    /// let data_u32 = type_map_opt.get::<u32, _>("one").map(|one| one.copied());
    /// let data_u64 = type_map_opt.get::<u64, _>("two").map(|two| two.copied());
    ///
    /// assert_eq!(Some(Some(1)), data_u32);
    /// assert_eq!(Some(None), data_u64);
    /// ```
    pub fn deserialize_map_opt<'de, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMapOpt<K, BoxDT>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapOptVisitor::<K, BoxDT, UnknownEntriesNone>::new(self);
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
        self.fn_seeds
            .get(type_key)
            .ok_or_else(|| self.unknown_type_error(type_key))
    }

    pub(crate) fn deserialize_opt_seed<E>(
        &self,
        type_key: &K,
    ) -> Result<&BoxFnSeed<Option<BoxDT>>, E>
    where
        E: serde::de::Error,
    {
        self.fn_opt_seeds
            .get(type_key)
            .ok_or_else(|| self.unknown_type_error(type_key))
    }

    fn unknown_type_error<E>(&self, type_key: &K) -> E
    where
        E: serde::de::Error,
    {
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
    }

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
    ///     three: 3\n\
    ///     ",
    /// );
    ///
    /// let type_map: TypeMap<String, _, _> = type_reg
    ///     .deserialize_map_with_unknowns::<'_, serde_yaml::Value, _, _>(deserializer)
    ///     .unwrap();
    /// let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    /// let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    ///
    /// println!("{data_u32}, {data_u64}"); // prints "1, 2"
    ///
    /// assert_eq!(
    ///     Some(serde_yaml::Value::Number(serde_yaml::Number::from(3u64))),
    ///     type_map.get_unknown_entry("three").cloned(),
    /// );
    /// ```
    pub fn deserialize_map_with_unknowns<'de, ValueT, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMap<K, BoxDT, UnknownEntriesSome<ValueT>>, E>
    where
        K: serde::de::Deserialize<'de> + 'de + 'static,
        ValueT: Clone + Debug + Eq + DeserializeOwned + 'static,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapVisitor::<K, BoxDT, BoxFnSeed<ValueT>>::new(
            self,
            BoxFnSeed::new(Self::deserialize_value::<ValueT>),
        );
        deserializer.deserialize_map(visitor)
    }

    /// Deserializes a map of arbitrary values into a [`TypeMapOpt`].
    ///
    /// Each type must be registered in this type registry before attempting to
    /// deserialize the type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMapOpt, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new();
    /// type_reg.register::<u32>(String::from("one"));
    /// type_reg.register::<u64>(String::from("two"));
    ///
    /// // This may be any deserializer.
    /// let deserializer = serde_yaml::Deserializer::from_str(
    ///     "---\n\
    ///     one: 1\n\
    ///     two: null\n\
    ///     three: 3\n\
    ///     ",
    /// );
    ///
    /// let type_map_opt: TypeMapOpt<String, _, _> = type_reg
    ///     .deserialize_map_opt_with_unknowns::<'_, serde_yaml::Value, _, _>(deserializer)
    ///     .unwrap();
    /// let data_u32 = type_map_opt.get::<u32, _>("one").map(|one| one.copied());
    /// let data_u64 = type_map_opt.get::<u64, _>("two").map(|two| two.copied());
    ///
    /// assert_eq!(Some(Some(1)), data_u32);
    /// assert_eq!(Some(None), data_u64);
    ///
    /// assert_eq!(
    ///     Some(Some(serde_yaml::Value::Number(serde_yaml::Number::from(
    ///         3u64
    ///     )))),
    ///     type_map_opt
    ///         .get_unknown_entry("three")
    ///         .map(|three| three.cloned()),
    /// );
    /// ```
    pub fn deserialize_map_opt_with_unknowns<'de, ValueT, D, E>(
        &'de self,
        deserializer: D,
    ) -> Result<TypeMapOpt<K, BoxDT, UnknownEntriesSome<ValueT>>, E>
    where
        K: serde::de::Deserialize<'de> + 'de + 'static,
        ValueT: Clone + Debug + Eq + DeserializeOwned + 'static,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        let visitor = TypeMapOptVisitor::<K, BoxDT, BoxFnSeed<Option<ValueT>>>::new(
            self,
            BoxFnSeed::new(Self::deserialize_opt_value::<ValueT>),
        );
        deserializer.deserialize_map(visitor)
    }

    pub(crate) fn deserialize_seed_opt(&self, type_key: &K) -> Option<&BoxFnSeed<BoxDT>> {
        self.fn_seeds.get(type_key)
    }

    pub(crate) fn deserialize_opt_seed_opt(
        &self,
        type_key: &K,
    ) -> Option<&BoxFnSeed<Option<BoxDT>>> {
        self.fn_opt_seeds.get(type_key)
    }
}

impl<K, BoxDT> Default for TypeReg<K, BoxDT>
where
    K: Eq + Hash + Debug,
{
    fn default() -> Self {
        Self {
            fn_seeds: Map::default(),
            fn_opt_seeds: Map::default(),
        }
    }
}

impl<K, BoxDT> Debug for TypeReg<K, BoxDT>
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

impl<K, BoxDT> Deref for TypeReg<K, BoxDT>
where
    K: Eq + Hash + Debug,
{
    type Target = Map<K, BoxFnSeed<BoxDT>>;

    fn deref(&self) -> &Self::Target {
        &self.fn_seeds
    }
}

impl<K, BoxDT> DerefMut for TypeReg<K, BoxDT>
where
    K: Eq + Hash + Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fn_seeds
    }
}

// Used by [`serde_tagged`] to select which [`DeserializeSeed`] function to use.
impl<'r, 'de, K, BoxDT> SeedFactory<'de, K> for &'r TypeReg<K, BoxDT>
where
    K: Clone + Eq + Hash + Debug + 'de + 'static,
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

#[cfg(test)]
mod tests {
    use std::fmt;

    use serde::{Deserialize, Serialize};

    use crate::untagged::{BoxDataTypeDowncast, BoxDtDisplay, TypeMap, TypeMapOpt, TypeReg};

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

    #[test]
    fn deserialize_map_new_typed() {
        let mut type_reg = TypeReg::<String, BoxDtDisplay>::new_typed();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<u64>(String::from("two"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: 3\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map: TypeMap<String, BoxDtDisplay> =
            type_reg.deserialize_map(deserializer).unwrap();

        let data_u32 = type_map.get::<u32, _>("one").copied();
        let data_u64 = type_map.get::<u64, _>("two").copied();
        let data_a = type_map.get::<A, _>("three").copied();

        assert_eq!(Some(1u32), data_u32);
        assert_eq!(Some(2u64), data_u64);
        assert_eq!(Some(A(3)), data_a);
    }

    #[test]
    fn deserialize_map_with_capacity_typed() {
        let mut type_reg = TypeReg::<String, BoxDtDisplay>::with_capacity_typed(3);
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<u64>(String::from("two"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: 3\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map: TypeMap<String, BoxDtDisplay> =
            type_reg.deserialize_map(deserializer).unwrap();

        let data_u32 = type_map.get::<u32, _>("one").copied();
        let data_u64 = type_map.get::<u64, _>("two").copied();
        let data_a = type_map.get::<A, _>("three").copied();

        assert_eq!(Some(1u32), data_u32);
        assert_eq!(Some(2u64), data_u64);
        assert_eq!(Some(A(3)), data_a);
    }

    #[cfg(feature = "ordered")]
    #[test]
    fn deserialize_single_has_good_error_message_when_type_not_registered() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let deserializer = serde_yaml::Deserializer::from_str("two: 2");
        let error = type_reg.deserialize_single(deserializer).unwrap_err();
        assert_eq!(
            r#"Type key `"two"` not registered in type registry.
Available types are:

- "one"
- "three"

"#,
            format!("{error}")
        );
    }

    #[cfg(feature = "ordered")]
    #[test]
    fn deserialize_map_opt_has_good_error_message_when_type_not_registered() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let deserializer = serde_yaml::Deserializer::from_str("two: 2");
        let error = type_reg.deserialize_map_opt(deserializer).unwrap_err();
        assert_eq!(
            r#"Type key `"two"` not registered in type registry.
Available types are:

- "one"
- "three"

"#,
            format!("{error}")
        );
    }

    #[test]
    fn deserialize_map_with_unknown_entries_yaml() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
            one: 1\n\
            two: 2\n\
            three: 3\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map = type_reg
            .deserialize_map_with_unknowns::<'_, serde_yaml::Value, _, _>(deserializer)
            .unwrap();

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
        let mut type_reg = TypeReg::<String>::new();
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
        let type_map = type_reg
            .deserialize_map_with_unknowns::<'_, serde_json::Value, _, _>(&mut deserializer)
            .unwrap();

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
    fn deserialize_map_opt() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<u64>(String::from("two"));
        type_reg.register::<A>(String::from("three"));

        let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: null\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map_opt: TypeMapOpt<String> = type_reg.deserialize_map_opt(deserializer).unwrap();

        let data_u32 = type_map_opt.get::<u32, _>("one").map(|one| one.copied());
        let data_u64 = type_map_opt.get::<u64, _>("two").map(|two| two.copied());
        let data_a = type_map_opt
            .get::<A, _>("three")
            .map(|three| three.copied());

        assert_eq!(Some(Some(1u32)), data_u32);
        assert_eq!(Some(Some(2u64)), data_u64);
        assert_eq!(Some(None), data_a);
    }

    #[test]
    fn deserialize_map_opt_with_unknown_entries_yaml() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));
        type_reg.register::<u8>(String::from("four"));

        let serialized = "---\n\
            one: 1\n\
            two: 2\n\
            three: 3\n\
            four: null\n\
            five: null\n\
        ";

        let deserializer = serde_yaml::Deserializer::from_str(serialized);
        let type_map_opt = type_reg
            .deserialize_map_opt_with_unknowns::<'_, serde_yaml::Value, _, _>(deserializer)
            .unwrap();

        let one = type_map_opt.get::<u32, _>("one").map(|one| one.copied());
        let two = type_map_opt
            .get_unknown_entry("two")
            .map(|two| two.cloned());
        let three = type_map_opt
            .get::<A, _>("three")
            .map(|three| three.copied());
        let four = type_map_opt.get::<u8, _>("four").map(|four| four.copied());
        let five = type_map_opt
            .get_unknown_entry("five")
            .map(|five| five.cloned());

        assert_eq!(Some(Some(1u32)), one);
        assert_eq!(Some(Some(A(3))), three);
        assert_eq!(Some(None::<u8>), four);

        assert_eq!(
            two,
            Some(Some(serde_yaml::Value::Number(serde_yaml::Number::from(
                2u64
            ))))
        );
        assert_eq!(five, Some(None));
        assert_eq!(2, type_map_opt.unknown_entries().len());
    }

    #[test]
    fn deserialize_map_opt_with_unknown_entries_json() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));
        type_reg.register::<A>(String::from("three"));
        type_reg.register::<u8>(String::from("four"));

        let serialized = r#"
            {
                "one": 1,
                "two": 2,
                "three": 3,
                "four": null,
                "five": null
            }
        "#;

        let mut deserializer = serde_json::Deserializer::from_str(serialized);
        let type_map_opt = type_reg
            .deserialize_map_opt_with_unknowns::<'_, serde_json::Value, _, _>(&mut deserializer)
            .unwrap();

        let one = type_map_opt.get::<u32, _>("one").map(|one| one.copied());
        let two = type_map_opt
            .get_unknown_entry("two")
            .map(|two| two.cloned());
        let three = type_map_opt
            .get::<A, _>("three")
            .map(|three| three.copied());
        let four = type_map_opt.get::<u8, _>("four").map(|four| four.copied());
        let five = type_map_opt
            .get_unknown_entry("five")
            .map(|five| five.cloned());

        assert_eq!(Some(Some(1u32)), one);
        assert_eq!(Some(Some(A(3))), three);
        assert_eq!(Some(None::<u8>), four);

        assert_eq!(
            two,
            Some(Some(serde_json::Value::Number(serde_json::Number::from(
                2u64
            ))))
        );
        assert_eq!(five, Some(None));
        assert_eq!(2, type_map_opt.unknown_entries().len());
    }

    #[test]
    fn with_capacity() {
        let type_reg = TypeReg::<String>::default();
        assert_eq!(0, type_reg.capacity());

        let type_reg = TypeReg::<String>::with_capacity(5);
        assert!(type_reg.capacity() >= 5);
    }

    #[test]
    fn deref_mut() {
        let mut type_reg = TypeReg::<String>::new();
        assert!(type_reg.get_mut("one").is_none())
    }

    #[test]
    fn debug() {
        let mut type_reg = TypeReg::new();
        type_reg.register::<A>("one");

        assert_eq!(r#"{"one": ".."}"#, format!("{type_reg:?}"));
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);

    impl fmt::Display for A {
        #[cfg_attr(coverage_nightly, no_coverage)]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }

    #[test]
    fn a_coverage() {
        let a = Clone::clone(&A(0));
        assert_eq!("A(0)", format!("{a:?}"));
        assert!(serde_yaml::to_string(&a).is_ok());
    }
}
