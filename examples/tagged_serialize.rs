use serde::{Deserialize, Serialize};
use type_reg::tagged::TypeMap;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_map = TypeMap::new();
    type_map.insert("one", Box::new(1u32));
    type_map.insert("two", Box::new(2u64));
    type_map.insert("three", Box::new(A(3)));

    println!("{}", serde_yaml::to_string(&type_map).unwrap());
}
