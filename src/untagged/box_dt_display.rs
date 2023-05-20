use std::{
    fmt::{self, Display},
    ops::{Deref, DerefMut},
};

use serde::Serialize;

use crate::{
    untagged::{BoxDataTypeDowncast, DataType, DataTypeDisplay, DataTypeWrapper, FromDataType},
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

    /// Returns the inner `Box<dyn DataTypeDisplay>`.
    pub fn into_inner(self) -> Box<dyn DataTypeDisplay> {
        self.0
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

impl<T> FromDataType<T> for BoxDtDisplay
where
    T: DataType + Display,
{
    fn from(t: T) -> BoxDtDisplay {
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
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use crate::untagged::{BoxDataTypeDowncast, DataTypeWrapper};

    use super::BoxDtDisplay;

    #[test]
    fn clone() {
        let box_dt_display = BoxDtDisplay::new(1u32);
        let mut box_dt_display_clone = Clone::clone(&box_dt_display);

        *BoxDataTypeDowncast::<u32>::downcast_mut(&mut box_dt_display_clone).unwrap() = 2;

        assert_eq!(
            Some(1u32),
            BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt_display).copied()
        );
        assert_eq!(
            Some(2u32),
            BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt_display_clone).copied()
        );
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug() {
        let box_dt_display = BoxDtDisplay::new(1u32);

        assert_eq!(r#"BoxDtDisplay("..")"#, format!("{box_dt_display:?}"));
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug() {
        let box_dt_display = BoxDtDisplay::new(1u32);

        assert_eq!("BoxDtDisplay(1)", format!("{box_dt_display:?}"));
    }

    #[test]
    fn display() {
        let box_dt_display = BoxDtDisplay::new(1u32);

        assert_eq!("1", format!("{box_dt_display}"));
    }

    #[test]
    fn deref() {
        let box_dt_display = BoxDtDisplay::new(1u32);
        let _data_type = Deref::deref(&box_dt_display);
    }

    #[test]
    fn deref_mut() {
        let mut box_dt_display = BoxDtDisplay::new(1u32);
        let _data_type = DerefMut::deref_mut(&mut box_dt_display);
    }

    #[test]
    fn serialize() -> Result<(), serde_yaml::Error> {
        let box_dt_display = BoxDtDisplay::new(1u32);
        let data_type_wrapper: &dyn DataTypeWrapper = &box_dt_display;

        assert_eq!("1\n", serde_yaml::to_string(data_type_wrapper)?);
        Ok(())
    }
}
