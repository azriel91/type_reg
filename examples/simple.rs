use serde::{Deserialize, Serialize};
use type_reg::tagged::{TypeMap, TypeReg};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct A(u32);

fn main() {
    let mut type_map = TypeMap::new();
    type_map.insert("one", Box::new(1u32));
    type_map.insert("two", Box::new(2u64));
    type_map.insert("three", Box::new(A(3)));

    println!("serde_yaml::to_string(&type_map):");
    println!("{}", serde_yaml::to_string(&type_map).unwrap());

    let mut type_reg = TypeReg::new();
    type_reg.register::<u32>();
    type_reg.register::<u64>();
    type_reg.register::<A>();

    let deserializer = serde_yaml::Deserializer::from_str("u32: 1");
    let data_type_u32 = type_reg.deserialize_single(deserializer).unwrap();
    let data_type_u32 = data_type_u32.downcast_ref::<u32>().copied();
    println!(r#"type_reg.deserialize_single(..) // "u32: 1""#);
    println!("-> {data_type_u32:?}\n");

    let serialized = "---\n\
        one:   { u32: 1 }\n\
        two:   { u64: 2 }\n\
        three: { 'simple::A': 3 }\n\
        ";

    let deserializer = serde_yaml::Deserializer::from_str(serialized);
    let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();

    let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    let data_a = type_map.get::<A, _>("three").copied().unwrap();

    println!("{data_u32}, {data_u64}, {data_a:?}");
}
