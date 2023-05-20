//! Type registry and map that does not serialize the type tag.
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
//! one: 1
//! two: 2
//! ```
//!
//! At runtime, deserialization relies on the key provided with type
//! registration matching the key of the value.
//!
//! # Examples
//!
//! ```rust
//! use type_reg::untagged::{TypeMap, TypeReg};
//!
//! let mut type_reg = TypeReg::<String>::new();
//! type_reg.register::<u32>(String::from("one"));
//! type_reg.register::<u64>(String::from("two"));
//!
//! // This may be any deserializer.
//! let deserializer = serde_yaml::Deserializer::from_str(
//!     "---\n\
//!     one: 1\n\
//!     two: 2\n\
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
    box_data_type_downcast::BoxDataTypeDowncast, box_dt::BoxDt, box_dt_display::BoxDtDisplay,
    data_type::DataType, data_type_display::DataTypeDisplay, data_type_wrapper::DataTypeWrapper,
    from_data_type::FromDataType, type_map::TypeMap, type_map_visitor::TypeMapVisitor,
    type_reg::TypeReg,
};

mod box_data_type_downcast;
mod box_dt;
mod box_dt_display;
mod data_type;
mod data_type_display;
mod data_type_wrapper;
mod from_data_type;
mod type_map;
mod type_map_visitor;
mod type_reg;
