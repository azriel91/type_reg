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

    fn downcast_ref<R>(&self) -> Option<&R>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
        Self: Sized;

    fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
        Self: Sized;

    fn inner(&self) -> &dyn DataType;

    fn inner_mut(&mut self) -> &mut dyn DataType;
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

    fn downcast_ref<R>(&self) -> Option<&R>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        <dyn Any>::downcast_ref::<R>(&self.0)
    }

    fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Clone + serde::Serialize + Send + Sync + 'static,
    {
        <dyn Any>::downcast_mut::<R>(&mut self.0)
    }

    fn inner(&self) -> &dyn DataType {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut dyn DataType {
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

    fn downcast_ref<R>(&self) -> Option<&R>
    where
        R: Clone + std::fmt::Debug + serde::Serialize + Send + Sync + 'static,
        Self: Sized;

    fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Clone + std::fmt::Debug + serde::Serialize + Send + Sync + 'static,
        Self: Sized;

    fn debug(&self) -> &dyn std::fmt::Debug;

    fn inner(&self) -> &dyn DataType;

    fn inner_mut(&mut self) -> &mut dyn DataType;
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

    fn downcast_ref<R>(&self) -> Option<&R>
    where
        R: Clone + std::fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0.downcast_ref::<R>()
    }

    fn downcast_mut<R>(&mut self) -> Option<&mut R>
    where
        R: Clone + std::fmt::Debug + serde::Serialize + Send + Sync + 'static,
    {
        self.0.downcast_mut::<R>()
    }

    fn debug(&self) -> &dyn std::fmt::Debug {
        &self.0
    }

    fn inner(&self) -> &dyn DataType {
        &self.0
    }

    fn inner_mut(&mut self) -> &mut dyn DataType {
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
