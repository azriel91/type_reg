//! Type registry and map that use an externally serialized type tag.
//!
//! This means for a given map:
//!
//! ```text,ignore
//! { "key": 1u32 }
//! ```
//!
//! The serialized form will be similar to the following YAML example:
//!
//! ```yaml
//! ---
//! key:
//!   u32: 1
//! ```

pub use self::{
    data_type::DataType, type_map::TypeMap, type_map_visitor::TypeMapVisitor, type_reg::TypeReg,
};

mod data_type;
mod type_map;
mod type_map_visitor;
mod type_reg;
