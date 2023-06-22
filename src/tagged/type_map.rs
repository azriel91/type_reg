use std::{
    borrow::Borrow,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{
    common::{UnknownEntries, UnknownEntriesNone, UnknownEntriesSome},
    tagged::DataType,
};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map of types that can be serialized / deserialized.
#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct TypeMap<K, UnknownEntriesT = UnknownEntriesNone>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    /// Underlying map.
    inner: Map<K, Box<dyn DataType>>,
    /// Unknown entries encountered during deserialization.
    #[serde(skip_serializing)]
    unknown_entries: Map<K, <UnknownEntriesT as UnknownEntries>::ValueT>,
}

impl<K> TypeMap<K, UnknownEntriesNone>
where
    K: Eq + Hash,
{
    // Creates an empty `TypeMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: Map::new(),
            unknown_entries: Map::new(),
        }
    }

    /// Creates an empty `TypeMap` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    /// let type_map = TypeMap::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Map::with_capacity(capacity),
            unknown_entries: Map::new(),
        }
    }

    /// Returns the underlying map.
    pub fn into_inner(self) -> Map<K, Box<dyn DataType>> {
        self.inner
    }
}

impl<
    K,
    #[cfg(not(feature = "debug"))] ValueT: Clone + PartialEq + Eq,
    #[cfg(feature = "debug")] ValueT: Clone + std::fmt::Debug + PartialEq + Eq,
> TypeMap<K, UnknownEntriesSome<ValueT>>
where
    K: Eq + Hash,
{
    /// Returns the underlying map and unknown entries.
    pub fn into_inner(self) -> (Map<K, Box<dyn DataType>>, Map<K, ValueT>) {
        (self.inner, self.unknown_entries)
    }
}

impl<K, UnknownEntriesT> TypeMap<K, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    // Creates an empty `TypeMap`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMap;
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// ```
    pub fn new_typed() -> Self {
        Self {
            inner: Map::new(),
            unknown_entries: Map::new(),
        }
    }

    /// Creates an empty `TypeMap` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMap;
    /// let type_map = TypeMap::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity_typed(capacity: usize) -> Self {
        Self {
            inner: Map::with_capacity(capacity),
            unknown_entries: Map::new(),
        }
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// If there is an entry, but the data type does not match, `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let one = type_map.get::<u32, _>("one").copied();
    /// assert_eq!(Some(1), one);
    /// ```
    #[cfg(not(feature = "debug"))]
    pub fn get<R, Q>(&self, q: &Q) -> Option<&R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.get(q).and_then(|n| n.downcast_ref::<R>())
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// If there is an entry, but the data type does not match, `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let one = type_map.get::<u32, _>("one").copied();
    /// assert_eq!(Some(1), one);
    /// ```
    #[cfg(feature = "debug")]
    pub fn get<R, Q>(&self, q: &Q) -> Option<&R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.get(q).and_then(|n| n.downcast_ref::<R>())
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// If there is an entry, but the data type does not match, `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let mut one = type_map.get_mut::<u32, _>("one");
    /// one.as_mut().map(|n| **n += 1);
    /// assert_eq!(Some(2), one.copied());
    /// ```
    #[cfg(not(feature = "debug"))]
    pub fn get_mut<R, Q>(&mut self, q: &Q) -> Option<&mut R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.get_mut(q).and_then(|n| n.downcast_mut::<R>())
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map’s key type, but `Hash` and
    /// `Eq` on the borrowed form must match those for the key type.
    ///
    /// If there is an entry, but the data type does not match, `None` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::tagged::TypeMap;
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let mut one = type_map.get_mut::<u32, _>("one");
    /// one.as_mut().map(|n| **n += 1);
    /// assert_eq!(Some(2), one.copied());
    #[cfg(feature = "debug")]
    pub fn get_mut<R, Q>(&mut self, q: &Q) -> Option<&mut R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.get_mut(q).and_then(|n| n.downcast_mut::<R>())
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[cfg(not(feature = "debug"))]
    pub fn insert<R>(&mut self, k: K, r: R) -> Option<Box<dyn DataType>>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.insert(k, Box::new(r))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[cfg(feature = "debug")]
    pub fn insert<R>(&mut self, k: K, r: R) -> Option<Box<dyn DataType>>
    where
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.inner.insert(k, Box::new(r))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_raw(&mut self, k: K, v: Box<dyn DataType>) -> Option<Box<dyn DataType>> {
        self.inner.insert(k, v)
    }
}

impl<K, UnknownEntriesT> Clone for TypeMap<K, UnknownEntriesT>
where
    K: Clone + Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    fn clone(&self) -> Self {
        let mut type_map = TypeMap::<K, UnknownEntriesT> {
            inner: Map::with_capacity(self.inner.len()),
            unknown_entries: Map::with_capacity(self.unknown_entries.len()),
        };
        self.inner.iter().for_each(|(k, v)| {
            let value = dyn_clone::clone_box(v);
            type_map.insert_raw(k.clone(), value);
        });
        self.unknown_entries.iter().for_each(|(k, v)| {
            let k = k.clone();
            let v = v.clone();
            type_map.unknown_entries.insert(k, v);
        });
        type_map
    }
}

impl<K, UnknownEntriesT> Default for TypeMap<K, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    fn default() -> Self {
        Self {
            inner: Map::default(),
            unknown_entries: Map::new(),
        }
    }
}

impl<K, UnknownEntriesT> Deref for TypeMap<K, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    type Target = Map<K, Box<dyn DataType>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, UnknownEntriesT> DerefMut for TypeMap<K, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K> fmt::Debug for TypeMap<K, UnknownEntriesNone>
where
    K: Eq + Hash + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.inner.iter().for_each(|(k, resource)| {
            // At runtime, we are unable to determine if the resource is `Debug`.
            #[cfg(not(feature = "debug"))]
            let value = &"..";

            #[cfg(feature = "debug")]
            let value = &resource;

            let type_name = resource.as_ref().type_name();
            let debug_value = crate::TypedValue {
                r#type: type_name,
                value,
            };

            debug_map.key(&k);
            debug_map.value(&debug_value);
        });

        debug_map.finish()
    }
}

struct InnerWrapper<'inner, K>
where
    K: Eq + Hash + fmt::Debug,
{
    inner: &'inner Map<K, Box<dyn DataType>>,
}

impl<'inner, K> fmt::Debug for InnerWrapper<'inner, K>
where
    K: Eq + Hash + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.inner.iter().for_each(|(k, resource)| {
            // At runtime, we are unable to determine if the resource is `Debug`.
            #[cfg(not(feature = "debug"))]
            let value = &"..";

            #[cfg(feature = "debug")]
            let value = &resource;

            let type_name = resource.as_ref().type_name();
            let debug_value = crate::TypedValue {
                r#type: type_name,
                value,
            };

            debug_map.key(&k);
            debug_map.value(&debug_value);
        });

        debug_map.finish()
    }
}

impl<
    K,
    #[cfg(not(feature = "debug"))] ValueT: Clone + PartialEq + Eq,
    #[cfg(feature = "debug")] ValueT: Clone + std::fmt::Debug + PartialEq + Eq,
> fmt::Debug for TypeMap<K, UnknownEntriesSome<ValueT>>
where
    K: Eq + Hash + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TypeMap")
            .field("inner", &InnerWrapper { inner: &self.inner })
            .field("unknown_entries", &self.unknown_entries)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::{common::UnknownEntriesSome, tagged::TypeMap};

    #[cfg(feature = "ordered")]
    #[test]
    fn serialize() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", 1u32);
        type_map.insert("two", 2u64);
        type_map.insert("three", A(3));

        let serialized = serde_yaml::to_string(&type_map).expect("Failed to serialize `type_map`.");
        let expected = r#"one:
  u32: 1
two:
  u64: 2
three:
  type_reg::tagged::type_map::tests::A: 3
"#
        .to_string();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn clone() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        let mut type_map_clone = type_map.clone();
        type_map_clone.insert("one", A(2));

        assert_eq!(Some(A(1)), type_map.get("one").copied());
        assert_eq!(Some(A(2)), type_map_clone.get("one").copied());
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        assert_eq!(
            r#"{"one": TypedValue { type: "type_reg::tagged::type_map::tests::A", value: ".." }}"#,
            format!("{type_map:?}")
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        assert_eq!(
            r#"{"one": TypedValue { type: "type_reg::tagged::type_map::tests::A", value: A(1) }}"#,
            format!("{type_map:?}")
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug_with_unknown_entries_some() {
        let mut type_map = TypeMap::<&'static str, UnknownEntriesSome<()>>::default();
        type_map.insert("one", A(1));

        assert_eq!(
            "TypeMap { \
                inner: {\
                    \"one\": TypedValue { type: \"type_reg::tagged::type_map::tests::A\", value: A(1) }}, \
                unknown_entries: {} \
            }",
            format!("{type_map:?}")
        );
    }

    #[test]
    fn into_inner_unknown_entries_none() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        let mut inner = type_map.into_inner();
        let one = inner
            .get_mut("one")
            .and_then(|n| n.downcast_mut::<A>())
            .copied();

        assert_eq!(Some(A(1)), one);
    }

    #[test]
    fn into_inner_unknown_entries_some() {
        let mut type_map = TypeMap::<&'static str, UnknownEntriesSome<()>>::default();
        type_map.insert("one", A(1));

        let (mut inner, unknown_entries) = type_map.into_inner();
        let one = inner
            .get_mut("one")
            .and_then(|n| n.downcast_mut::<A>())
            .copied();

        assert_eq!(Some(A(1)), one);
        assert!(unknown_entries.is_empty());
    }

    #[test]
    fn get_mut() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        let one = type_map.get_mut::<A, _>("one").copied();
        let two = type_map.get_mut::<A, _>("two").copied();
        let three = type_map.get_mut::<u32, _>("one").copied();

        assert_eq!(Some(A(1)), one);
        assert_eq!(None, two);
        assert_eq!(None, three);
    }

    #[test]
    fn with_capacity() {
        let type_map = TypeMap::<&str>::default();
        assert_eq!(0, type_map.capacity());

        let type_map = TypeMap::<&str>::with_capacity(5);
        assert!(type_map.capacity() >= 5);
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);
}
