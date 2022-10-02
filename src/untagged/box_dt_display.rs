use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::Serialize;

use crate::untagged::{DataType, DataTypeDisplay};

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
