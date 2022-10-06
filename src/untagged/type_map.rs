use std::{
    borrow::Borrow,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::untagged::{BoxDt, DataTypeWrapper};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map of types that can be serialized / deserialized.
#[derive(serde::Serialize)]
pub struct TypeMap<K, BoxDT = BoxDt>(Map<K, BoxDT>)
where
    K: Eq + Hash;

impl<K> TypeMap<K, BoxDt>
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
    /// use type_reg::untagged::TypeMap;
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// ```
    pub fn new() -> Self {
        Self(Map::new())
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
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Map::with_capacity(capacity))
    }
}

impl<K, BoxDT> TypeMap<K, BoxDT>
where
    K: Eq + Hash,
    BoxDT: DataTypeWrapper,
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
        Self(Map::new())
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
        Self(Map::with_capacity(capacity))
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
    /// use type_reg::untagged::TypeMap;
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
        self.0.get(q).and_then(DataTypeWrapper::downcast_ref::<R>)
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
    /// use type_reg::untagged::TypeMap;
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
        self.0.get(q).and_then(DataTypeWrapper::downcast_ref::<R>)
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
    /// use type_reg::untagged::TypeMap;
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
        self.0
            .get_mut(q)
            .and_then(DataTypeWrapper::downcast_mut::<R>)
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
    /// use type_reg::untagged::TypeMap;
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
        self.0
            .get_mut(q)
            .and_then(DataTypeWrapper::downcast_mut::<R>)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[cfg(not(feature = "debug"))]
    pub fn insert<R>(&mut self, k: K, r: R) -> Option<BoxDT>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.0.insert(k, BoxDT::new(r))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[cfg(feature = "debug")]
    pub fn insert<R>(&mut self, k: K, r: R) -> Option<BoxDT>
    where
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0.insert(k, BoxDT::new(r))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_raw(&mut self, k: K, v: BoxDT) -> Option<BoxDT> {
        self.0.insert(k, v)
    }
}

impl<K, BoxDT> Clone for TypeMap<K, BoxDT>
where
    K: Clone + Eq + Hash,
    BoxDT: DataTypeWrapper,
{
    fn clone(&self) -> Self {
        let mut type_map = TypeMap::<K, BoxDT>::with_capacity_typed(self.0.len());
        self.0.iter().for_each(|(k, v)| {
            let value = dyn_clone::clone_box(&**v);
            type_map.insert_raw(k.clone(), BoxDT::from_box(value));
        });
        type_map
    }
}

impl<K, BoxDT> Default for TypeMap<K, BoxDT>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(Map::default())
    }
}

impl<K, BoxDT> Deref for TypeMap<K, BoxDT>
where
    K: Eq + Hash,
{
    type Target = Map<K, BoxDT>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, BoxDT> DerefMut for TypeMap<K, BoxDT>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, BoxDT> fmt::Debug for TypeMap<K, BoxDT>
where
    K: Eq + Hash + fmt::Debug,
    BoxDT: DataTypeWrapper,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.0.iter().for_each(|(k, resource)| {
            // At runtime, we are unable to determine if the resource is `Debug`.
            #[cfg(not(feature = "debug"))]
            let value = &"..";

            #[cfg(feature = "debug")]
            let value = resource.debug();

            let type_name = resource.type_name();
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

#[cfg(test)]
mod tests {
    use crate::untagged::TypeMap;
    use serde::{Deserialize, Serialize};

    #[cfg(feature = "ordered")]
    #[test]
    fn serialize() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", 1u32);
        type_map.insert("two", 2u64);
        type_map.insert("three", A(3));

        let serialized = serde_yaml::to_string(&type_map).expect("Failed to serialize `type_map`.");
        let expected = r#"one: 1
two: 2
three: 3
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
            r#"{"one": TypedValue { type: "type_reg::untagged::type_map::tests::A", value: ".." }}"#,
            format!("{type_map:?}")
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        assert_eq!(
            r#"{"one": TypedValue { type: "type_reg::untagged::type_map::tests::A", value: A(1) }}"#,
            format!("{type_map:?}")
        );
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
