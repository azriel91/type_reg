use std::{
    borrow::Borrow,
    collections::HashMap,
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

use crate::{tagged::DataType, TypeNameLit};

/// Map of types that can be serialized / deserialized.
#[derive(serde::Serialize)]
pub struct TypeMap<K>(HashMap<K, Box<dyn DataType>>)
where
    K: Eq + Hash;

impl<K> TypeMap<K>
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
    /// let mut type_reg = TypeMap::<&'static str>::new();
    /// ```
    pub fn new() -> Self {
        Self(HashMap::new())
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
    /// let type_reg = TypeMap::<&'static str>::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(HashMap::with_capacity(capacity))
    }

    ///
    #[cfg(not(feature = "debug"))]
    pub fn get<R, Q>(&self, q: &Q) -> Option<&R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: serde::Serialize + Send + Sync + 'static,
    {
        self.0.get(q).and_then(|n| n.downcast_ref::<R>())
    }

    #[cfg(feature = "debug")]
    pub fn get<R, Q>(&self, q: &Q) -> Option<&R>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
        R: fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0.get(q).and_then(|n| n.downcast_ref::<R>())
    }
}

impl<K> Default for TypeMap<K>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Self(HashMap::default())
    }
}

impl<K> Deref for TypeMap<K>
where
    K: Eq + Hash,
{
    type Target = HashMap<K, Box<dyn DataType>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K> DerefMut for TypeMap<K>
where
    K: Eq + Hash,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// This is used in the Debug impl, but for some reason rustc warns the fields
// are not used.
#[allow(dead_code)]
#[derive(Debug)]
struct TypedValue<'a> {
    r#type: TypeNameLit,
    value: &'a dyn fmt::Debug,
}

// This is used in the Debug impl, but for some reason rustc warns the fields
// are not used.
#[allow(dead_code)]
#[derive(Debug)]
struct UntypedValue<'a> {
    value: &'a dyn fmt::Debug,
}

impl<K> fmt::Debug for TypeMap<K>
where
    K: Eq + Hash + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.0.iter().for_each(|(k, resource)| {
            // At runtime, we are unable to determine if the resource is `Debug`.
            #[cfg(not(feature = "debug"))]
            let value = &"..";

            #[cfg(feature = "debug")]
            let value = &resource;

            let type_name = resource.as_ref().type_name();
            let debug_value = TypedValue {
                r#type: type_name,
                value,
            };

            debug_map.key(&k);
            debug_map.value(&debug_value);
        });

        debug_map.finish()
    }
}
