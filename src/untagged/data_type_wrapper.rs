use std::{
    any::Any,
    ops::{Deref, DerefMut},
};

use dyn_clone::DynClone;

use crate::{
    untagged::{BoxDt, DataType},
    TypeNameLit,
};

/// Trait to represent the stored type.
#[cfg(not(feature = "debug"))]
pub trait DataTypeWrapper:
    Deref<Target = dyn DataType> + DerefMut + erased_serde::Serialize
{
    fn new<T>(t: T) -> Self
    where
        T: Any + DynClone + erased_serde::Serialize + Send + Sync,
        Self: Sized;

    fn from_box(data_type: Box<dyn DataType>) -> Self
    where
        Self: Sized;

    fn type_name(&self) -> TypeNameLit;

    // Needed for `downcast_ref` in `TypeMap` to cast to the correct type.
    #[allow(clippy::borrowed_box)]
    fn inner(&self) -> &Box<dyn DataType>;

    // Needed for `downcast_mut` in `TypeMap` to cast to the correct type.
    #[allow(clippy::borrowed_box)]
    fn inner_mut(&mut self) -> &mut Box<dyn DataType>;
}

#[cfg(not(feature = "debug"))]
impl DataTypeWrapper for BoxDt {
    fn new<T>(t: T) -> Self
    where
        T: Any + DynClone + erased_serde::Serialize + Send + Sync,
    {
        Self(Box::new(t))
    }

    fn from_box(data_type: Box<dyn DataType>) -> Self {
        Self(data_type)
    }

    fn type_name(&self) -> TypeNameLit {
        DataType::type_name(&*self.0)
    }

    fn inner(&self) -> &Box<dyn DataType> {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Box<dyn DataType> {
        &mut self.0
    }
}

/// Trait to represent the stored type.
#[cfg(feature = "debug")]
pub trait DataTypeWrapper:
    Deref<Target = dyn DataType> + DerefMut + std::fmt::Debug + erased_serde::Serialize
{
    fn new<T>(t: T) -> Self
    where
        T: Any + DynClone + std::fmt::Debug + erased_serde::Serialize + Send + Sync,
        Self: Sized;

    fn from_box(data_type: Box<dyn DataType>) -> Self
    where
        Self: Sized;

    fn type_name(&self) -> TypeNameLit;

    // Needed for `downcast_ref` in `TypeMap` to cast to the correct type.
    #[allow(clippy::borrowed_box)]
    fn inner(&self) -> &Box<dyn DataType>;

    // Needed for `downcast_mut` in `TypeMap` to cast to the correct type.
    #[allow(clippy::borrowed_box)]
    fn inner_mut(&mut self) -> &mut Box<dyn DataType>;
}

#[cfg(feature = "debug")]
impl DataTypeWrapper for BoxDt {
    fn new<T>(t: T) -> Self
    where
        T: Any + DynClone + std::fmt::Debug + erased_serde::Serialize + Send + Sync,
        Self: Sized,
    {
        Self(Box::new(t))
    }

    fn from_box(data_type: Box<dyn DataType>) -> Self {
        Self(data_type)
    }

    fn type_name(&self) -> TypeNameLit {
        DataType::type_name(&*self.0)
    }

    fn inner(&self) -> &Box<dyn DataType> {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut Box<dyn DataType> {
        &mut self.0
    }
}

impl<'a> serde::Serialize for dyn DataTypeWrapper + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.inner(), serializer)
    }
}
