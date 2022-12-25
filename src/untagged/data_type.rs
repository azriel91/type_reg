use std::any::Any;

use downcast_rs::DowncastSync;
use dyn_clone::DynClone;

use crate::TypeNameLit;

/// Trait to represent the stored type.
#[cfg(all(not(feature = "debug"), not(feature = "resman")))]
pub trait DataType: DowncastSync + DynClone + erased_serde::Serialize {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(all(not(feature = "debug"), not(feature = "resman")))]
impl<T> DataType for T
where
    T: Any + DynClone + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

/// Trait to represent the stored type.
#[cfg(all(not(feature = "debug"), feature = "resman"))]
pub trait DataType: resman::Resource + DowncastSync + DynClone + erased_serde::Serialize {
    fn type_name(&self) -> TypeNameLit;
    fn upcast(self: Box<Self>) -> Box<dyn resman::Resource>;
}

#[cfg(all(not(feature = "debug"), feature = "resman"))]
impl<T> DataType for T
where
    T: Any + DynClone + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }

    fn upcast(self: Box<Self>) -> Box<dyn resman::Resource> {
        self
    }
}

/// Trait to represent the stored type.
#[cfg(all(feature = "debug", not(feature = "resman")))]
pub trait DataType: DowncastSync + DynClone + std::fmt::Debug + erased_serde::Serialize {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(all(feature = "debug", not(feature = "resman")))]
impl<T> DataType for T
where
    T: Any + DynClone + std::fmt::Debug + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

/// Trait to represent the stored type.
#[cfg(all(feature = "debug", feature = "resman"))]
pub trait DataType:
    resman::Resource + DowncastSync + DynClone + std::fmt::Debug + erased_serde::Serialize
{
    fn type_name(&self) -> TypeNameLit;
    fn upcast(self: Box<Self>) -> Box<dyn resman::Resource>;
}

#[cfg(all(feature = "debug", feature = "resman"))]
impl<T> DataType for T
where
    T: Any + DynClone + std::fmt::Debug + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }

    fn upcast(self: Box<Self>) -> Box<dyn resman::Resource> {
        self
    }
}

downcast_rs::impl_downcast!(sync DataType);
dyn_clone::clone_trait_object!(DataType);

impl<'a> serde::Serialize for dyn DataType + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
