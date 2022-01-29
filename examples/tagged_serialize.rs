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
    //   "tagged_serialize::A": 3
    // two:
    //   u64: 2
}
