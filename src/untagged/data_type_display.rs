use std::{any::Any, fmt};

use dyn_clone::DynClone;

use crate::untagged::DataType;

/// A [`DataType`] that is also [`Display`].
pub trait DataTypeDisplay: DataType + fmt::Display {}

#[cfg(not(feature = "debug"))]
impl<T> DataTypeDisplay for T where
    T: Any + DynClone + fmt::Display + erased_serde::Serialize + Send + Sync
{
}

#[cfg(feature = "debug")]
impl<T> DataTypeDisplay for T where
    T: Any + DynClone + fmt::Debug + fmt::Display + erased_serde::Serialize + Send + Sync
{
}

downcast_rs::impl_downcast!(sync DataTypeDisplay);
dyn_clone::clone_trait_object!(DataTypeDisplay);

impl<'a> serde::Serialize for dyn DataTypeDisplay + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self, serializer)
    }
}
