use serde::{Deserialize, Serialize};
use type_reg::{TypeMap, TypeReg};

#[derive(Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_map = TypeMap::new();
    type_map.insert("one", Box::new(1u32));
    type_map.insert("two", Box::new(2u64));
    type_map.insert("three", Box::new(A(3)));

    println!("{}", serde_yaml::to_string(&type_map).unwrap());

    let mut type_reg = TypeReg::new();
    type_reg.register::<u32>();
    type_reg.register::<u64>();
    type_reg.register::<A>();

    let data_type_u32 = type_reg
        .deserialize_untyped(serde_yaml::Deserializer::from_str("u32: 1"))
        .unwrap();
    let data_type_u32 = data_type_u32.downcast_ref::<u32>().copied();
    println!("{data_type_u32:?}\n");

    let serialized = "---\n\
        one:   { u32: 1 }\n\
        two:   { u64: 2 }\n\
        three: { 'simple::A': 3 }\n\
        ";

    let type_map: TypeMap<String> = type_reg
        .deserialize_map(serde_yaml::Deserializer::from_str(serialized))
        .unwrap();

    println!("{:#?}", type_map);
}
