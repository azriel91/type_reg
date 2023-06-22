# Changelog

## unreleased

* Add `untagged::TypeReg::with_unknown_entries` so unregistered values can be deserialized as a generic value type.
* Add `untagged::TypeMap::unknown_entries` to access entries that did not have a type registered for deserialization.


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
