use std::ops::{Deref, DerefMut};

use serde::Serialize;

use crate::{
    untagged::{BoxDataTypeDowncast, DataType, DataTypeWrapper, FromDataType},
    TypeNameLit,
};

/// Box of any type, with no additional trait constraints.
#[derive(Clone, Serialize)]
pub struct BoxDt(pub(crate) Box<dyn DataType>);

#[cfg(not(feature = "debug"))]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BoxDt").field(&"..").finish()
    }
}

#[cfg(feature = "debug")]
impl std::fmt::Debug for BoxDt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_tuple("BoxDt").field(&self.0).finish()
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

    /// Returns the inner `Box<dyn DataType>`.
    pub fn into_inner(self) -> Box<dyn DataType> {
        self.0
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

impl<T> FromDataType<T> for BoxDt
where
    T: DataType,
{
    fn from(t: T) -> BoxDt {
        BoxDt(Box::new(t))
    }
}

impl<T> BoxDataTypeDowncast<T> for BoxDt
where
    T: DataType,
{
    fn downcast_ref(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    fn downcast_mut(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}

impl DataTypeWrapper for BoxDt {
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

    use super::BoxDt;

    #[test]
    fn clone() {
        let box_dt = BoxDt::new(1u32);
        let mut box_dt_clone = Clone::clone(&box_dt);

        *BoxDataTypeDowncast::<u32>::downcast_mut(&mut box_dt_clone).unwrap() = 2;

        assert_eq!(
            Some(1u32),
            BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt).copied()
        );
        assert_eq!(
            Some(2u32),
            BoxDataTypeDowncast::<u32>::downcast_ref(&box_dt_clone).copied()
        );
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug() {
        let box_dt = BoxDt::new(1u32);

        assert_eq!(r#"BoxDt("..")"#, format!("{box_dt:?}"));
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug() {
        let box_dt = BoxDt::new(1u32);

        assert_eq!("BoxDt(1)", format!("{box_dt:?}"));
    }

    #[test]
    fn deref() {
        let box_dt = BoxDt::new(1u32);
        let _data_type = Deref::deref(&box_dt);
    }

    #[test]
    fn deref_mut() {
        let mut box_dt = BoxDt::new(1u32);
        let _data_type = DerefMut::deref_mut(&mut box_dt);
    }

    #[test]
    fn serialize() -> Result<(), serde_yaml::Error> {
        let box_dt = BoxDt::new(1u32);
        let data_type_wrapper: &dyn DataTypeWrapper = &box_dt;

        assert_eq!("1\n", serde_yaml::to_string(data_type_wrapper)?);
        Ok(())
    }
}
