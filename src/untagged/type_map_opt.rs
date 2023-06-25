use std::{
    borrow::Borrow,
    fmt::{self, Debug},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{
    common::{UnknownEntries, UnknownEntriesNone, UnknownEntriesSome},
    untagged::{BoxDataTypeDowncast, BoxDt, DataTypeWrapper, FromDataType},
};

#[cfg(not(feature = "ordered"))]
use std::collections::HashMap as Map;

#[cfg(feature = "ordered")]
use indexmap::IndexMap as Map;

/// Map of types that can be serialized / deserialized, values are optional.
///
/// Where [`TypeMap`] is a `Map<K, V>`, `TypeMapOpt` is a `Map<K, Option<V>>`.
///
/// [`TypeMap`]: crate::untagged::TypeMap
#[derive(serde::Serialize)]
#[serde(transparent)]
pub struct TypeMapOpt<K, BoxDT = BoxDt, UnknownEntriesT = UnknownEntriesNone>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    /// Underlying map.
    inner: Map<K, Option<BoxDT>>,
    /// Unknown entries encountered during deserialization.
    #[serde(skip_serializing)]
    unknown_entries: Map<K, Option<<UnknownEntriesT as UnknownEntries>::ValueT>>,
}

impl<K> TypeMapOpt<K, BoxDt>
where
    K: Eq + Hash,
{
    // Creates an empty `TypeMapOpt`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMapOpt;
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// ```
    pub fn new() -> Self {
        Self {
            inner: Map::new(),
            unknown_entries: Map::new(),
        }
    }

    /// Creates an empty `TypeMapOpt` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMapOpt;
    /// let type_map = TypeMapOpt::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Map::with_capacity(capacity),
            unknown_entries: Map::new(),
        }
    }
}

impl<K, BoxDT> TypeMapOpt<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash,
    BoxDT: DataTypeWrapper,
{
    /// Returns the underlying map.
    pub fn into_inner(self) -> Map<K, Option<BoxDT>> {
        self.inner
    }
}

impl<K, BoxDT, ValueT> TypeMapOpt<K, BoxDT, UnknownEntriesSome<ValueT>>
where
    K: Eq + Hash,
    BoxDT: DataTypeWrapper,
    ValueT: Clone + Debug + PartialEq + Eq,
{
    /// Returns the underlying map and unknown entries.
    pub fn into_inner(self) -> (Map<K, Option<BoxDT>>, Map<K, Option<ValueT>>) {
        (self.inner, self.unknown_entries)
    }

    /// Returns the entries that were unable to be deserialized.
    ///
    /// These are the entries from the source data for which no type was
    /// registered against the [`TypeReg`] used to deserialize that source data.
    ///
    /// [`TypeReg`]: crate::untagged::TypeReg
    pub fn unknown_entries(&self) -> &Map<K, Option<ValueT>> {
        &self.unknown_entries
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
    /// ## YAML
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMapOpt, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new().with_unknown_entries::<serde_yaml::Value>();
    ///
    /// let type_map = type_reg
    ///     .deserialize_map(serde_yaml::Deserializer::from_str("one: 1"))
    ///     .unwrap();
    ///
    /// let one = type_map.get_unknown_entry("one").cloned();
    ///
    /// assert_eq!(
    ///     one,
    ///     Some(serde_yaml::Value::Number(serde_yaml::Number::from(1u32)))
    /// );
    /// assert_eq!(1, type_map.unknown_entries().len());
    /// ```
    ///
    /// ## JSON
    ///
    /// ```rust
    /// use type_reg::untagged::{TypeMapOpt, TypeReg};
    ///
    /// let mut type_reg = TypeReg::<String>::new().with_unknown_entries::<serde_json::Value>();
    ///
    /// let type_map = type_reg
    ///     .deserialize_map(&mut serde_json::Deserializer::from_str(r#"{ "one": 1 }"#))
    ///     .unwrap();
    ///
    /// let one = type_map.get_unknown_entry("one").cloned();
    ///
    /// assert_eq!(
    ///     one,
    ///     Some(serde_json::Value::Number(serde_json::Number::from(1u32)))
    /// );
    /// assert_eq!(1, type_map.unknown_entries().len());
    /// ```
    pub fn get_unknown_entry<Q>(&self, q: &Q) -> Option<Option<&ValueT>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.unknown_entries()
            .get(q)
            .map(|value_opt| value_opt.as_ref())
    }

    /// Inserts an unknown entry into the map.
    ///
    /// This is only used during deserialization.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub(crate) fn insert_unknown_entry(
        &mut self,
        k: K,
        v: Option<ValueT>,
    ) -> Option<Option<ValueT>> {
        self.unknown_entries.insert(k, v)
    }
}

impl<K, BoxDT, UnknownEntriesT> TypeMapOpt<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash,
    BoxDT: DataTypeWrapper,
    UnknownEntriesT: UnknownEntries,
{
    // Creates an empty `TypeMapOpt`.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMapOpt;
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// ```
    pub fn new_typed() -> Self {
        Self {
            inner: Map::new(),
            unknown_entries: Map::new(),
        }
    }

    /// Creates an empty `TypeMapOpt` with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use type_reg::untagged::TypeMapOpt;
    /// let type_map = TypeMapOpt::<&'static str>::with_capacity(10);
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
    /// use type_reg::untagged::TypeMapOpt;
    ///
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// type_map.insert("one", Some(1u32));
    ///
    /// let one = type_map.get::<u32, _>("one").map(|one| one.copied());
    /// assert_eq!(Some(Some(1)), one);
    /// ```
    pub fn get<#[cfg(not(feature = "debug"))] R, #[cfg(feature = "debug")] R: Debug, Q>(
        &self,
        q: &Q,
    ) -> Option<Option<&R>>
    where
        K: Borrow<Q>,
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.inner
            .get(q)
            .map(|r| r.as_ref().and_then(BoxDataTypeDowncast::<R>::downcast_ref))
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
    /// use type_reg::untagged::TypeMapOpt;
    ///
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// type_map.insert("one", Some(1u32));
    ///
    /// let mut one_plus_one_opt = type_map.get_mut::<u32, _>("one");
    /// one_plus_one_opt
    ///     .as_mut()
    ///     .map(|mut one_plus_one| {
    ///         one_plus_one
    ///             .as_mut()
    ///             .map(|one| **one += 1);
    ///     });
    /// assert_eq!(Some(Some(&mut 2)), one_plus_one_opt);
    #[cfg(feature = "debug")]
    pub fn get_mut<#[cfg(not(feature = "debug"))] R, #[cfg(feature = "debug")] R: Debug, Q>(
        &mut self,
        q: &Q,
    ) -> Option<Option<&mut R>>
    where
        K: Borrow<Q>,
        BoxDT: BoxDataTypeDowncast<R>,
        Q: Hash + Eq + ?Sized,
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        self.inner
            .get_mut(q)
            .map(|r| r.as_mut().and_then(BoxDataTypeDowncast::<R>::downcast_mut))
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
    /// use type_reg::untagged::{BoxDataTypeDowncast, TypeMapOpt};
    ///
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// type_map.insert("one", Some(1u32));
    ///
    /// let boxed_one_opt = type_map.get_raw("one");
    /// let one = boxed_one_opt.map(|boxed_one| {
    ///     boxed_one
    ///         .and_then(|boxed_one| BoxDataTypeDowncast::<u32>::downcast_ref(boxed_one))
    ///         .copied()
    /// });
    /// assert_eq!(Some(Some(1)), one);
    /// ```
    pub fn get_raw<Q>(&self, q: &Q) -> Option<Option<&BoxDT>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(q).map(|box_dt| box_dt.as_ref())
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
    /// use type_reg::untagged::{BoxDataTypeDowncast, TypeMapOpt};
    ///
    /// let mut type_map = TypeMapOpt::<&'static str>::new();
    /// type_map.insert("one", Some(1u32));
    ///
    /// let boxed_one_opt = type_map.get_raw_mut("one");
    /// let one = boxed_one_opt.map(|boxed_one| {
    ///     boxed_one.and_then(|boxed_one| BoxDataTypeDowncast::<u32>::downcast_mut(boxed_one))
    /// });
    /// assert_eq!(Some(Some(1).as_mut()), one);
    ///
    /// if let Some(Some(one)) = one {
    ///     *one += 1;
    /// }
    ///
    /// let one_plus_one = type_map
    ///     .get::<u32, _>("one")
    ///     .map(|one_plus_one| one_plus_one.copied());
    /// assert_eq!(Some(Some(2)), one_plus_one);
    /// ```
    pub fn get_raw_mut<Q>(&mut self, q: &Q) -> Option<Option<&mut BoxDT>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get_mut(q).map(|box_dt| box_dt.as_mut())
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    #[cfg(feature = "debug")]
    pub fn insert<#[cfg(not(feature = "debug"))] R, #[cfg(feature = "debug")] R: Debug>(
        &mut self,
        k: K,
        r: Option<R>,
    ) -> Option<Option<BoxDT>>
    where
        BoxDT: FromDataType<R>,
    {
        self.inner
            .insert(k, r.map(<BoxDT as FromDataType<R>>::from))
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical.
    pub fn insert_raw(&mut self, k: K, v: Option<BoxDT>) -> Option<Option<BoxDT>> {
        self.inner.insert(k, v)
    }
}

impl<K, BoxDT, UnknownEntriesT> Clone for TypeMapOpt<K, BoxDT, UnknownEntriesT>
where
    K: Clone + Eq + Hash,
    BoxDT: DataTypeWrapper,
    UnknownEntriesT: UnknownEntries,
{
    fn clone(&self) -> Self {
        let mut type_map_opt = TypeMapOpt::<K, BoxDT, UnknownEntriesT> {
            inner: Map::with_capacity(self.inner.len()),
            unknown_entries: Map::with_capacity(self.unknown_entries.len()),
        };
        self.inner.iter().for_each(|(k, v)| {
            let value = v.as_ref().map(|box_dt| box_dt.clone());
            type_map_opt.insert_raw(k.clone(), value);
        });
        self.unknown_entries.iter().for_each(|(k, v)| {
            let k = k.clone();
            let v = v.as_ref().map(|value| value.clone());
            type_map_opt.unknown_entries.insert(k, v);
        });
        type_map_opt
    }
}

impl<K, BoxDT, UnknownEntriesT> Default for TypeMapOpt<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    fn default() -> Self {
        Self {
            inner: Map::default(),
            unknown_entries: Map::default(),
        }
    }
}

impl<K, BoxDT, UnknownEntriesT> Deref for TypeMapOpt<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    type Target = Map<K, Option<BoxDT>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, BoxDT, UnknownEntriesT> DerefMut for TypeMapOpt<K, BoxDT, UnknownEntriesT>
where
    K: Eq + Hash,
    UnknownEntriesT: UnknownEntries,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<K, BoxDT> Debug for TypeMapOpt<K, BoxDT, UnknownEntriesNone>
where
    K: Eq + Hash + Debug,
    BoxDT: DataTypeWrapper,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.inner.iter().for_each(|(k, resource_opt)| {
            // At runtime, we are unable to determine if the resource is `Debug`.
            let debug_value = resource_opt.as_ref().map(|resource| {
                let type_name = resource.type_name();

                #[cfg(not(feature = "debug"))]
                let value = &"..";

                #[cfg(feature = "debug")]
                let value = resource.debug();

                crate::TypedValue {
                    r#type: type_name,
                    value,
                }
            });

            debug_map.key(&k);
            debug_map.value(&debug_value);
        });

        debug_map.finish()
    }
}

struct InnerWrapper<'inner, K, BoxDT>
where
    K: Eq + Hash + Debug,
    BoxDT: DataTypeWrapper,
{
    inner: &'inner Map<K, Option<BoxDT>>,
}

impl<'inner, K, BoxDT> Debug for InnerWrapper<'inner, K, BoxDT>
where
    K: Eq + Hash + Debug,
    BoxDT: DataTypeWrapper,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.inner.iter().for_each(|(k, resource_opt)| {
            let debug_value = resource_opt.as_ref().map(|resource| {
                // At runtime, we are unable to determine if the resource is `Debug`.
                #[cfg(not(feature = "debug"))]
                let value = &"..";

                #[cfg(feature = "debug")]
                let value = resource.debug();

                let type_name = resource.type_name();
                crate::TypedValue {
                    r#type: type_name,
                    value,
                }
            });

            debug_map.key(&k);
            debug_map.value(&debug_value);
        });

        debug_map.finish()
    }
}

impl<K, BoxDT, ValueT> Debug for TypeMapOpt<K, BoxDT, UnknownEntriesSome<ValueT>>
where
    K: Eq + Hash + Debug,
    BoxDT: DataTypeWrapper,
    ValueT: Clone + Debug + PartialEq + Eq,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TypeMapOpt")
            .field("inner", &InnerWrapper { inner: &self.inner })
            .field("unknown_entries", &self.unknown_entries)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{self, Write};

    use serde::{Deserialize, Serialize};

    use crate::{
        common::UnknownEntriesSome,
        untagged::{BoxDataTypeDowncast, BoxDt, BoxDtDisplay, TypeMapOpt},
    };

    #[cfg(feature = "ordered")]
    #[test]
    fn serialize() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(1u32));
        type_map.insert("two", None::<u64>);
        type_map.insert("three", Some(A(3)));

        let serialized = serde_yaml::to_string(&type_map).expect("Failed to serialize `type_map`.");
        let expected = r#"one: 1
two: null
three: 3
"#
        .to_string();
        assert_eq!(expected, serialized);
    }

    #[test]
    fn clone() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));

        let mut type_map_clone = type_map.clone();
        type_map_clone.insert("one", Some(A(2)));

        assert_eq!(Some(Some(&A(1))), type_map.get("one"));
        assert_eq!(Some(Some(&A(2))), type_map_clone.get("one"));
    }

    #[test]
    fn into_inner() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));

        let index_map = type_map.into_inner();

        assert!(index_map.get("one").is_some());
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        assert_eq!(
            "{\
                \"one\": Some(TypedValue { \
                    type: \"type_reg::untagged::type_map_opt::tests::A\", \
                    value: \"..\" \
                }), \
                \"two\": None \
            }",
            format!("{type_map:?}")
        );
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug_with_unknown_entries_some() {
        let mut type_map = TypeMapOpt::<&'static str, BoxDt, UnknownEntriesSome<()>>::default();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        assert_eq!(
            "TypeMapOpt { \
                inner: {\
                    \"one\": Some(TypedValue { \
                        type: \"type_reg::untagged::type_map_opt::tests::A\", \
                        value: \"..\" \
                    }), \
                    \"two\": None \
                }, \
                unknown_entries: {} \
            }",
            format!("{type_map:?}")
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        assert_eq!(
            "{\
                \"one\": Some(TypedValue { type: \"type_reg::untagged::type_map_opt::tests::A\", value: A(1) }), \
                \"two\": None\
            }",
            format!("{type_map:?}")
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug_with_unknown_entries_some() {
        let mut type_map = TypeMapOpt::<&'static str, BoxDt, UnknownEntriesSome<()>>::default();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        assert_eq!(
            "TypeMapOpt { \
                inner: {\
                    \"one\": Some(TypedValue { type: \"type_reg::untagged::type_map_opt::tests::A\", value: A(1) }), \
                    \"two\": None\
                }, \
                unknown_entries: {} \
            }",
            format!("{type_map:?}")
        );
    }

    #[test]
    fn into_inner_unknown_entries_none() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        let mut inner = type_map.into_inner();
        let one = inner.get_mut("one").map(|box_dt_opt| {
            box_dt_opt
                .as_mut()
                .and_then(BoxDataTypeDowncast::<A>::downcast_mut)
                .copied()
        });
        let two = inner.get_mut("two").map(|box_dt_opt| {
            box_dt_opt
                .as_mut()
                .and_then(BoxDataTypeDowncast::<u64>::downcast_mut)
                .copied()
        });

        assert_eq!(Some(Some(A(1))), one);
        assert_eq!(Some(None::<u64>), two);
    }

    #[test]
    fn into_inner_unknown_entries_some() {
        let mut type_map = TypeMapOpt::<&'static str, BoxDt, UnknownEntriesSome<()>>::default();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", None::<u64>);

        let (mut inner, unknown_entries) = type_map.into_inner();
        let one = inner.get_mut("one").map(|box_dt_opt| {
            box_dt_opt
                .as_mut()
                .and_then(BoxDataTypeDowncast::<A>::downcast_mut)
                .copied()
        });
        let two = inner.get_mut("two").map(|box_dt_opt| {
            box_dt_opt
                .as_mut()
                .and_then(BoxDataTypeDowncast::<u64>::downcast_mut)
                .copied()
        });

        assert_eq!(Some(Some(A(1))), one);
        assert_eq!(Some(None::<u64>), two);
        assert!(unknown_entries.is_empty());
    }

    #[test]
    fn get_mut() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));
        type_map.insert("two", Some(ADisplay(2)));
        type_map.insert("three", None::<u64>);

        let one = type_map
            .get_mut::<A, _>("one")
            .map(Option::<&mut _>::copied);
        let two = type_map
            .get_mut::<ADisplay, _>("two")
            .map(Option::<&mut _>::copied);
        let three = type_map
            .get_mut::<u64, _>("three")
            .map(Option::<&mut _>::copied);
        let four = type_map
            .get_mut::<u32, _>("four")
            .map(Option::<&mut _>::copied);

        assert_eq!(Some(Some(A(1))), one);
        assert_eq!(Some(Some(ADisplay(2))), two);
        assert_eq!(Some(None::<u64>), three);
        assert_eq!(None, four);
    }

    #[test]
    fn get_raw() {
        let mut type_map = TypeMapOpt::<&'static str>::new();
        type_map.insert("one", Some(1u32));
        let boxed_one_opt = type_map.get_raw("one");

        let one = boxed_one_opt.map(|boxed_one| {
            boxed_one
                .and_then(BoxDataTypeDowncast::<u32>::downcast_ref)
                .copied()
        });

        assert_eq!(Some(Some(1)), one);
    }

    #[test]
    fn get_raw_mut() {
        let mut type_map = TypeMapOpt::<&'static str>::new();
        type_map.insert("one", Some(1u32));

        let boxed_one_opt = type_map.get_raw_mut("one");
        let one = boxed_one_opt
            .map(|boxed_one| boxed_one.and_then(BoxDataTypeDowncast::<u32>::downcast_mut));
        assert_eq!(Some(Some(1).as_mut()), one);

        if let Some(Some(one)) = one {
            *one += 1;
        }

        let one_plus_one = type_map
            .get::<u32, _>("one")
            .map(|one_plus_one| one_plus_one.copied());
        assert_eq!(Some(Some(2)), one_plus_one);
    }

    #[test]
    fn with_capacity() {
        let type_map = TypeMapOpt::<&str>::default();
        assert_eq!(0, type_map.capacity());

        let type_map = TypeMapOpt::<&str>::with_capacity(5);
        assert!(type_map.capacity() >= 5);
    }

    #[test]
    fn deref_mut() {
        let mut type_map = TypeMapOpt::new();
        type_map.insert("one", Some(A(1)));

        if let Some(Some(v)) = type_map.values_mut().next() {
            if let Some(a) = BoxDataTypeDowncast::<A>::downcast_mut(v) {
                a.0 = 2;
            }
        };

        let one_plus_one = type_map
            .get::<A, _>("one")
            .map(|one_plus_one| one_plus_one.copied());
        assert_eq!(Some(Some(A(2))), one_plus_one);
    }

    #[test]
    fn display() -> fmt::Result {
        let mut type_map = TypeMapOpt::<_, BoxDtDisplay>::new_typed();
        type_map.insert("one", Some(ADisplay(1)));

        let formatted = type_map
            .iter()
            .try_fold(String::with_capacity(64), |mut s, (k, v)| {
                if let Some(v) = v {
                    write!(&mut s, "{k}: {v}")?;
                }
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
