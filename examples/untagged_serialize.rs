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
