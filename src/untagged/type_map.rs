use std::{
    borrow::Borrow,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::untagged::{BoxDataTypeDowncast, BoxDt, DataTypeWrapper, FromDataType};

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

    /// Returns the underlying map.
    pub fn into_inner(self) -> Map<K, BoxDT> {
        self.0
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
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.0
            .get(q)
            .and_then(BoxDataTypeDowncast::<R>::downcast_ref)
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
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0
            .get(q)
            .and_then(BoxDataTypeDowncast::<R>::downcast_ref)
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
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.0
            .get_mut(q)
            .and_then(BoxDataTypeDowncast::<R>::downcast_mut)
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
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0
            .get_mut(q)
            .and_then(BoxDataTypeDowncast::<R>::downcast_mut)
    }

    /// Returns a reference to the boxed value corresponding to the key.
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
    /// use type_reg::untagged::{BoxDataTypeDowncast, TypeMap};
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let boxed_one = type_map.get_raw("one");
    /// let one = boxed_one
    ///     .and_then(|boxed_one| BoxDataTypeDowncast::<u32>::downcast_ref(boxed_one))
    ///     .copied();
    /// assert_eq!(Some(1), one);
    /// ```
    pub fn get_raw<Q>(&self, q: &Q) -> Option<&BoxDT>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get(q)
    }

    /// Returns a mutable reference to the boxed value corresponding to the key.
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
    /// use type_reg::untagged::{BoxDataTypeDowncast, TypeMap};
    ///
    /// let mut type_map = TypeMap::<&'static str>::new();
    /// type_map.insert("one", 1u32);
    ///
    /// let boxed_one = type_map.get_raw_mut("one");
    /// let one = boxed_one.and_then(|boxed_one| BoxDataTypeDowncast::<u32>::downcast_mut(boxed_one));
    /// assert_eq!(Some(1).as_mut(), one);
    ///
    /// if let Some(one) = one {
    ///     *one += 1;
    /// }
    ///
    /// let one_plus_one = type_map.get::<u32, _>("one").copied();
    /// assert_eq!(Some(2), one_plus_one);
    /// ```
    pub fn get_raw_mut<Q>(&mut self, q: &Q) -> Option<&mut BoxDT>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get_mut(q)
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
        BoxDT: FromDataType<R>,
    {
        self.0.insert(k, <BoxDT as FromDataType<R>>::from(r))
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
        BoxDT: FromDataType<R>,
    {
        self.0.insert(k, <BoxDT as FromDataType<R>>::from(r))
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
            let value = v.clone();
            type_map.insert_raw(k.clone(), value);
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
    use std::fmt::{self, Write};

    use serde::{Deserialize, Serialize};

    use crate::untagged::{BoxDataTypeDowncast, BoxDtDisplay, TypeMap};

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

    #[test]
    fn into_inner() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        let index_map = type_map.into_inner();

        assert!(index_map.get("one").is_some());
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
        type_map.insert("two", ADisplay(2));

        let one = type_map.get_mut::<A, _>("one").copied();
        let two = type_map.get_mut::<ADisplay, _>("two").copied();
        let three = type_map.get_mut::<u32, _>("one").copied();

        assert_eq!(Some(A(1)), one);
        assert_eq!(Some(ADisplay(2)), two);
        assert_eq!(None, three);
    }

    #[test]
    fn get_raw() {
        let mut type_map = TypeMap::<&'static str>::new();
        type_map.insert("one", 1u32);
        let boxed_one = type_map.get_raw("one");

        let one = boxed_one
            .and_then(BoxDataTypeDowncast::<u32>::downcast_ref)
            .copied();

        assert_eq!(Some(1), one);
    }

    #[test]
    fn get_raw_mut() {
        let mut type_map = TypeMap::<&'static str>::new();
        type_map.insert("one", 1u32);

        let boxed_one = type_map.get_raw_mut("one");
        let one = boxed_one.and_then(BoxDataTypeDowncast::<u32>::downcast_mut);
        assert_eq!(Some(1).as_mut(), one);

        if let Some(one) = one {
            *one += 1;
        }

        let one_plus_one = type_map.get::<u32, _>("one").copied();
        assert_eq!(Some(2), one_plus_one);
    }

    #[test]
    fn with_capacity() {
        let type_map = TypeMap::<&str>::default();
        assert_eq!(0, type_map.capacity());

        let type_map = TypeMap::<&str>::with_capacity(5);
        assert!(type_map.capacity() >= 5);
    }

    #[test]
    fn deref_mut() {
        let mut type_map = TypeMap::new();
        type_map.insert("one", A(1));

        if let Some(v) = type_map.values_mut().next() {
            if let Some(a) = BoxDataTypeDowncast::<A>::downcast_mut(v) {
                a.0 = 2;
            }
        };

        let one = type_map.get::<A, _>("one").copied();
        assert_eq!(Some(A(2)), one);
    }

    #[test]
    fn display() -> fmt::Result {
        let mut type_map = TypeMap::<_, BoxDtDisplay>::new_typed();
        type_map.insert("one", ADisplay(1));

        let formatted = type_map
            .iter()
            .try_fold(String::with_capacity(64), |mut s, (k, v)| {
                write!(&mut s, "{k}: {v}")?;
                Ok(s)
            })?;

        assert_eq!("one: 1", formatted);
        Ok(())
    }

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct A(u32);

    #[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
    struct ADisplay(u32);

    impl fmt::Display for ADisplay {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }
}
