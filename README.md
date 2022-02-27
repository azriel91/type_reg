# 🗂️ type_reg

[![Crates.io](https://img.shields.io/crates/v/type_reg.svg)](https://crates.io/crates/type_reg)
[![docs.rs](https://img.shields.io/docsrs/type_reg)](https://docs.rs/type_reg)
[![CI](https://github.com/azriel91/type_reg/workflows/CI/badge.svg)](https://github.com/azriel91/type_reg/actions/workflows/ci.yml)
[![Coverage Status](https://codecov.io/gh/azriel91/type_reg/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/type_reg)

Serializable map of any type.

This library provides a map that can store any serializable type, and retrieve it as the strong type. Serialization and deserialization is also done without requiring the consumer to implement custom serde logic.

## Usage

Add the following to `Cargo.toml`

```toml
type_reg = { version = "0.2.0", features = ["tagged"] }
type_reg = { version = "0.2.0", features = ["untagged"] }

# Values must impl Debug, and TypeMap's Debug impl will
# print the debug string of each value.
type_reg = { version = "0.2.0", features = ["debug"] }

# Use insertion order for TypeMap and TypeReg iteration order.
type_reg = { version = "0.2.0", features = ["ordered"] }
```

### Tagged Type Registry

⚠️ **Note:** This uses [`std::any::type_name`] internally, which is not stable.

#### Serialization

```rust
use serde::{Deserialize, Serialize};
use type_reg::tagged::TypeMap;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_map = TypeMap::new();
    type_map.insert("one", 1u32);
    type_map.insert("two", 2u64);
    type_map.insert("three", A(3));

    println!("{}", serde_yaml::to_string(&type_map).unwrap());

    // ---
    // one:
    //   u32: 1
    // three:
    //   "rust_out::A": 3
    // two:
    //   u64: 2
}
```

#### Deserialization

```rust
use serde::{Deserialize, Serialize};
use type_reg::tagged::{TypeMap, TypeReg};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_reg = TypeReg::new();
    type_reg.register::<u32>();
    type_reg.register::<u64>();
    type_reg.register::<A>();

    let serialized = "---\n\
        one:   { u32: 1 }\n\
        two:   { u64: 2 }\n\
        three: { 'tagged_deserialize_map::A': 3 }\n\
        ";

    let deserializer = serde_yaml::Deserializer::from_str(serialized);
    let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();

    let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    let data_a = type_map.get::<A, _>("three").copied().unwrap();

    println!("{data_u32}, {data_u64}, {data_a:?}");

    // 1, 2, A(3)
}
```

### Untagged Type Registry

#### Serialization

```rust
use serde::{Deserialize, Serialize};
use type_reg::untagged::TypeMap;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_map = TypeMap::new();
    type_map.insert("one", 1u32);
    type_map.insert("two", 2u64);
    type_map.insert("three", A(3));

    println!("{}", serde_yaml::to_string(&type_map).unwrap());

    // ---
    // two: 2
    // one: 1
    // three: 3
}
```

#### Deserialization

```rust
use serde::{Deserialize, Serialize};
use type_reg::untagged::{TypeMap, TypeReg};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_reg = TypeReg::<String>::new();
    type_reg.register::<u32>(String::from("one"));
    type_reg.register::<u64>(String::from("two"));
    type_reg.register::<A>(String::from("three"));

    let serialized = "---\n\
        one: 1\n\
        two: 2\n\
        three: 3\n\
        ";

    let deserializer = serde_yaml::Deserializer::from_str(serialized);
    let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();

    let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    let data_a = type_map.get::<A, _>("three").copied().unwrap();

    println!("{data_u32}, {data_u64}, {data_a:?}");

    // 1, 2, A(3)
}
```

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
[`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html
