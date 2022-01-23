//! Serializable map of any type.

pub use crate::{type_map::TypeMap, type_reg::TypeReg};

pub(crate) use data_type::DataType;
pub(crate) use type_name_lit::TypeNameLit;

mod data_type;
mod de;
mod type_map;
mod type_name_lit;
mod type_reg;
