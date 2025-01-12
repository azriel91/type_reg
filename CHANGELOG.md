# Changelog

## 0.8.0 (2025-01-12)

* Update dependency versions.


## 0.7.1 (2024-12-30)

* Don't require `TypeReg` to live as long as the source data, only as long as deserialization. ([#14][#14])
* Update dependency versions.


[#14]: https://github.com/azriel91/type_reg/pull/14


## 0.7.0 (2023-12-30)

* Update dependency versions.


## 0.6.2 (2023-09-23)

* Update dependency versions.


## 0.6.1 (2023-09-16)

* Update dependency versions.
* Update coverage attribute due to `cargo-llvm-cov` upgrade.


## 0.6.0 (2023-06-27)

* Add `untagged::TypeMap::unknown_entries` to access entries that did not have a type registered for deserialization.
* Add `untagged::TypeReg::deserialize_map_with_unknowns` so unregistered values can be deserialized as a generic value type instead of returning a deserialization failure, or silently dropping entries.
* Add `untagged::TypeMapOpt`, where entries are stored as `Option<T>`.
* Add `untagged::TypeReg::deserialize_map_opt` to deserialize a `TypeMapOpt`.
* Add `untagged::TypeReg::deserialize_map_opt_with_unknowns` to be able to deserialize unregistered `Option<T>` as a generic value type instead of returning a deserialization failure, or silently dropping entries.
* `indexmap` is updated from `1.9.3` to `2.0.0`.


## 0.5.2 (2023-06-12)

* Add `untagged::TypeMap::{get_raw, get_raw_mut}`.


## 0.5.1 (2023-05-20)

* Update dependency versions.
* Replace `IntoBoxDataType` with `FromDataType` so external crates can use their own box DT.


## 0.5.0 (2022-12-26)

* Update dependency versions.
* Add `TypeMap::into_inner`.
* Support upcasting from `Box<dyn DataType>` to `Box<dyn resman::Resource>`.
* Add `BoxDtDisplay::into_inner`.
* Add `BoxDt::into_inner`.


## 0.4.0 (2022-10-09)

* In `untagged`, genericize `TypeReg` and `TypeMap`, so stored type may have different trait bounds.
* `TypeMap` defaults to storing `BoxDt`, which has `Clone`, serialization, and optionally `Debug` constraints.
* `TypeMap` may store `BoxDtDisplay`, which adds the `Display` constraint.
* ***Breaking:*** Previously, `Box<dyn DataType>` may be downcasted to `T` through `data.downcast_ref::<T>()`. Now, one needs to use `BoxDataTypeDowncast::<T>::downcast_ref(box_dt)`;


## 0.3.1 (2022-09-03)

* Implement `Debug` for `TypeReg`.


## 0.3.0 (2022-02-27)

* Implement `Clone` for `TypeMap`.


## 0.2.0 (2022-02-27)

* Add `"ordered"` feature to iterate and serialize in insertion order.


## 0.1.0 (2022-01-29)

* Add `tagged::TypeReg` and `tagged::TypeMap`.
* Add `untagged::TypeReg` and `untagged::TypeMap`.
