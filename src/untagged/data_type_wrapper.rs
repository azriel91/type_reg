use crate::{untagged::DataType, TypeNameLit};

/// Trait to represent the stored type.
#[cfg(not(feature = "debug"))]
pub trait DataTypeWrapper: erased_serde::Serialize {
    fn type_name(&self) -> TypeNameLit;

    fn clone(&self) -> Self
    where
        Self: Sized;

    fn inner(&self) -> &dyn DataType;
}

/// Trait to represent the stored type.
#[cfg(feature = "debug")]
pub trait DataTypeWrapper: std::fmt::Debug + erased_serde::Serialize {
    fn type_name(&self) -> TypeNameLit;

    fn clone(&self) -> Self
    where
        Self: Sized;

    fn debug(&self) -> &dyn std::fmt::Debug;

    fn inner(&self) -> &dyn DataType;
}

impl serde::Serialize for dyn DataTypeWrapper + '_ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        erased_serde::serialize(self.inner(), serializer)
    }
}
