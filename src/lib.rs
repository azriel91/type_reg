//! Serializable map of any type.

#[cfg(feature = "tagged")]
pub mod tagged;

pub use crate::type_name_lit::TypeNameLit;

mod type_name_lit;
