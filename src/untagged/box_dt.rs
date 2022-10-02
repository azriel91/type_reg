use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::untagged::DataType;

/// Box of any type, with no additional trait constraints.
#[derive(Clone, Serialize)]
pub struct BoxDt(pub(crate) Box<dyn DataType>);

#[cfg(feature = "debug")]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        (*self.0).fmt(f)
    }
}

#[cfg(not(feature = "debug"))]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BoxDt").field(&"..").finish()
    }
}

impl BoxDt {
    /// Returns a new `BoxDt` wrapper around the provided type.
    pub fn new<T>(t: T) -> Self
    where
        T: DataType,
    {
        Self(Box::new(t))
    }
}

impl Deref for BoxDt {
    type Target = dyn DataType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxDt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
