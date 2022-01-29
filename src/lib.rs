//! Serializable map of any type.
//!
//! This library provides a map that can store any serializable type, and
//! retrieve it as the strong type. Serialization and deserialization is also
//! done without requiring the consumer to implement custom serde logic.
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! # any combination of
//! type_reg = { version = "0.1.0", features = ["tagged"] }
//! type_reg = { version = "0.1.0", features = ["untagged"] }
//! type_reg = { version = "0.1.0", features = ["debug"] }
//! ```
//!
//! ### Tagged Type Registry
//!
//! #### Serialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use type_reg::tagged::TypeMap;
//!
//! #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//! struct A(u32);
//!
//! let mut type_map = TypeMap::new();
//! type_map.insert("one", 1u32);
//! type_map.insert("two", 2u64);
//! type_map.insert("three", A(3));
//!
//! println!("{}", serde_yaml::to_string(&type_map).unwrap());
//!
//! // ---
//! // one:
//! //   u32: 1
//! // three:
//! //   "tagged_serialize::A": 3
//! // two:
//! //   u64: 2
//! ```
//!
//! #### Deserialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use type_reg::tagged::{TypeMap, TypeReg};
//!
//! #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//! struct A(u32);
//!
//! let mut type_reg = TypeReg::new();
//! type_reg.register::<u32>();
//! type_reg.register::<u64>();
//! type_reg.register::<A>();
//!
//! let serialized = "---\n\
//!     one:   { u32: 1 }\n\
//!     two:   { u64: 2 }\n\
//!     three: { 'rust_out::A': 3 }\n\
//!     ";
//!
//! let deserializer = serde_yaml::Deserializer::from_str(serialized);
//! let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
//!
//! let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
//! let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
//! let data_a = type_map.get::<A, _>("three").copied().unwrap();
//!
//! println!("{data_u32}, {data_u64}, {data_a:?}");
//!
//! // 1, 2, A(3)
//! ```
//!
//! ### Untagged Type Registry
//!
//! #### Serialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use type_reg::untagged::TypeMap;
//!
//! #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//! struct A(u32);
//!
//! let mut type_map = TypeMap::new();
//! type_map.insert("one", 1u32);
//! type_map.insert("two", 2u64);
//! type_map.insert("three", A(3));
//!
//! println!("{}", serde_yaml::to_string(&type_map).unwrap());
//!
//! // ---
//! // two: 2
//! // one: 1
//! // three: 3
//! ```
//!
//! #### Deserialization
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use type_reg::untagged::{TypeMap, TypeReg};
//!
//! #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
//! struct A(u32);
//!
//! let mut type_reg = TypeReg::<String>::new();
//! type_reg.register::<u32>(String::from("one"));
//! type_reg.register::<u64>(String::from("two"));
//! type_reg.register::<A>(String::from("three"));
//!
//! let serialized = "---\n\
//!     one: 1\n\
//!     two: 2\n\
//!     three: 3\n\
//!     ";
//!
//! let deserializer = serde_yaml::Deserializer::from_str(serialized);
//! let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();
//!
//! let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
//! let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
//! let data_a = type_map.get::<A, _>("three").copied().unwrap();
//!
//! println!("{data_u32}, {data_u64}, {data_a:?}");
//!
//! // 1, 2, A(3)
//! ```

#[cfg(feature = "tagged")]
pub mod tagged;
#[cfg(feature = "untagged")]
pub mod untagged;

pub use crate::type_name_lit::TypeNameLit;

mod type_name_lit;