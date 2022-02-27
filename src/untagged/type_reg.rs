use std::{
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use serde_tagged::de::{BoxFnSeed, SeedFactory};

use crate::untagged::{DataType, TypeMap, TypeMapVisitor};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map from a given key to logic to deserialize a type.
pub struct TypeReg<K>(Map<K, BoxFnSeed<Box<dyn DataType>>>)
where
    K: Eq + Hash + fmt::Debug;

impl<K> TypeReg<K>
where
    K: Eq + Hash + fmt::Debug,
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
        Self(Map::new())
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
        Self(Map::with_capacity(capacity))
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
    {
        fn deserialize<'de, R>(
            deserializer: &mut dyn erased_serde::Deserializer<'de>,
        ) -> Result<Box<dyn DataType>, erased_serde::Error>
        where
            R: serde::de::DeserializeOwned + DataType + 'static,
        {
            Ok(Box::new(R::deserialize(deserializer)?))
        }

        self.0.insert(key, BoxFnSeed::new(deserialize::<R>));
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
    pub fn deserialize_map<'de, D, E>(&'de self, deserializer: D) -> Result<TypeMap<K>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
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
    pub fn deserialize_single<'de, D, E>(&self, deserializer: D) -> Result<Box<dyn DataType>, E>
    where
        K: serde::de::Deserialize<'de> + 'de,
        D: serde::de::Deserializer<'de, Error = E>,
        E: serde::de::Error,
    {
        serde_tagged::de::external::deserialize(deserializer, self)
    }

    pub(crate) fn deserialize_seed<E>(
        &self,
        type_key: &K,
    ) -> Result<&BoxFnSeed<Box<dyn DataType>>, E>
    where
        E: serde::de::Error,
    {
        self.0.get(type_key).ok_or_else(|| {
            use std::fmt::Write;
            let mut message = String::with_capacity(256);
            write!(
                message,
                "Type key `{type_key:?}` not registered in type registry."
            )
            .expect("Failed to write error message");

            message.push_str("\nAvailable types are:\n\n");
            let mut message = self
                .0
                .keys()
                .try_fold(message, |mut message, key| {
                    writeln!(message, "- {key:?}")?;
                    Result::<_, fmt::Error>::Ok(message)
                })
                .expect("Failed to write error message");
            message.push_str("\n");

            serde::de::Error::custom(message)
        })
    }
}

impl<K> Default for TypeReg<K>
where
    K: Eq + Hash + fmt::Debug,
{
    fn default() -> Self {
        Self(Map::default())
    }
}

impl<K> Deref for TypeReg<K>
where
    K: Eq + Hash + fmt::Debug,
{
    type Target = Map<K, BoxFnSeed<Box<dyn DataType>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for TypeReg<K>
where
    K: Eq + Hash + fmt::Debug,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// Used by [`serde_tagged`] to select which [`DeserializeSeed`] function to use.
impl<'r, 'de, K> SeedFactory<'de, K> for &'r TypeReg<K>
where
    K: Eq + Hash + fmt::Debug + 'de,
{
    type Seed = &'r BoxFnSeed<Box<dyn DataType>>;
    type Value = Box<dyn DataType>;

    fn seed<E>(self, type_tag: K) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        self.deserialize_seed(&type_tag)
    }
}

#[cfg(test)]
mod tests {
    use crate::untagged::{TypeMap, TypeReg};
    use serde::{Deserialize, Serialize};

    #[test]
    fn deserialize_single() {
        let mut type_reg = TypeReg::<String>::new();
        type_reg.register::<u32>(String::from("one"));

        let deserializer = serde_yaml::Deserializer::from_str("one: 1");
        let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
        let data_u32 = data_u32.downcast_ref::<u32>().copied();

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

 at line 1 column 4"#,
                format!("{error}")
            );
        } else {
            panic!("Expected `deserialize_single` to return error.");
        }
    }

    #[test]
    fn with_capacity() {
        let type_reg = TypeReg::<String>::default();
        assert_eq!(0, type_reg.0.capacity());

        let type_reg = TypeReg::<String>::with_capacity(5);
        assert!(type_reg.0.capacity() >= 5);
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);
}
