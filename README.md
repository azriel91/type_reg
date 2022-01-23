# 🗂️ type_reg

[![Crates.io](https://img.shields.io/crates/v/type_reg.svg)](https://crates.io/crates/type_reg)
[![docs.rs](https://img.shields.io/docsrs/type_reg)](https://docs.rs/type_reg)
![CI](https://github.com/azriel91/type_reg/workflows/CI/badge.svg)
[![Coverage Status](https://codecov.io/gh/azriel91/type_reg/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/type_reg)

Serializable map of any type.

This library provides a map that can store any serializable type, and retrieve it as the strong type. Serialization and deserialization is also done without requiring the consumer to implement custom serde logic.


## Usage

Add the following to `Cargo.toml`

```toml
type_reg = "0.1.0"

# or
type_reg = { version = "0.1.0", features = ["debug"] }
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
