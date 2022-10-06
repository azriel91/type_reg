use std::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use serde::Serialize;

use crate::{
    untagged::{BoxDataTypeDowncast, DataType, DataTypeDisplay, DataTypeWrapper, IntoBoxDataType},
    TypeNameLit,
};

/// Box of any type, with no additional trait constraints.
#[cfg_attr(feature = "debug", derive(Debug))]
#[derive(Clone, Serialize)]
pub struct BoxDtDisplay(pub(crate) Box<dyn DataTypeDisplay>);

#[cfg(not(feature = "debug"))]
impl std::fmt::Debug for BoxDtDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BoxDtDisplay").field(&"..").finish()
    }
}

impl BoxDtDisplay {
    /// Returns a new `BoxDtDisplay` wrapper around the provided type.
    pub fn new<T>(t: T) -> Self
    where
        T: DataType + Display,
    {
        Self(Box::new(t))
    }
}

impl Deref for BoxDtDisplay {
    type Target = dyn DataTypeDisplay;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxDtDisplay {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for BoxDtDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> IntoBoxDataType<BoxDtDisplay> for T
where
    T: DataType + Display,
{
    fn into(t: Self) -> BoxDtDisplay {
        BoxDtDisplay(Box::new(t))
    }
}

impl<T> BoxDataTypeDowncast<T> for BoxDtDisplay
where
    T: DataType + Display,
{
    fn downcast_ref(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}

impl DataTypeWrapper for BoxDtDisplay {
    fn type_name(&self) -> TypeNameLit {
        DataType::type_name(&*self.0)
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[cfg(feature = "debug")]
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
