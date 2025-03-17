//! Type registry and map that use an externally serialized type tag.
//!
//! This means for a given map:
//!
//! ```text,ignore
//! {
//!     "one": 1u32,
//!     "two": 2u64,
//! }
//! ```
//!
//! The serialized form will be similar to the following YAML example:
//!
//! ```yaml
//! ---
//! one:
//!   u32: 1
//! two:
//!   u64: 2
//! ```
//!
//! At runtime, deserialization relies on the type tag matching the qualified
//! type name of the registered type.
//!
//! # Examples
//!
//! ```rust
//! use type_reg::tagged::{TypeMap, TypeReg};
//!
//! let mut type_reg = TypeReg::new();
//! type_reg.register::<u32>();
//! type_reg.register::<u64>();
//!
//! // This may be any deserializer.
//! let deserializer = serde_yaml_ng::Deserializer::from_str(
//!     "---\n\
//!     one: { u32: 1 }\n\
//!     two: { u64: 2 }\n\
//!     ",
//! );
//!
//! let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
//! let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
//! let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
//!
//! println!("{data_u32}, {data_u64}"); // prints "1, 2"
//! ```

pub use self::{
    data_type::DataType, type_map::TypeMap, type_map_visitor::TypeMapVisitor, type_reg::TypeReg,
};

mod data_type;
mod type_map;
mod type_map_visitor;
mod type_reg;
