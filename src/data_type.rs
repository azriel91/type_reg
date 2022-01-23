use std::any::Any;

use downcast_rs::DowncastSync;
use serde_tagged::util::erased::SerializeErased;

use crate::TypeNameLit;

/// Trait to represent the stored type.
#[cfg(not(feature = "debug"))]
pub trait DataType: DowncastSync + erased_serde::Serialize + 'static {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(not(feature = "debug"))]
impl<T> DataType for T
where
    T: Any + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

/// Trait to represent the stored type.
#[cfg(feature = "debug")]
pub trait DataType: DowncastSync + std::fmt::Debug + erased_serde::Serialize + 'static {
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(feature = "debug")]
impl<T> DataType for T
where
    T: Any + std::fmt::Debug + erased_serde::Serialize + Send + Sync,
{
    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

downcast_rs::impl_downcast!(sync DataType);

impl<'a> serde::Serialize for dyn DataType + 'a {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // As tag we simply use the ID provided by our `DataType` trait.
        // To serialize our trait object value (without the tag) we actually
        // need to call `erased_serde::serialize`. We can do this by wrapping
        // the object in `SerializeErased`.
        // The `serialize` method of `serde_erased::ser::external` will apply
        // our type-id as tag to the trait-object.
        serde_tagged::ser::external::serialize(
            serializer,
            &DataType::type_name(self),
            &SerializeErased(self),
        )
    }
}
