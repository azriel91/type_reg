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

    let deserializer = serde_yaml_ng::Deserializer::from_str(serialized);
    let type_map: TypeMap<String> = type_reg.deserialize_map(deserializer).unwrap();

    let data_u32 = type_map.get::<u32, _>("one").copied().unwrap();
    let data_u64 = type_map.get::<u64, _>("two").copied().unwrap();
    let data_a = type_map.get::<A, _>("three").copied().unwrap();

    println!("{data_u32}, {data_u64}, {data_a:?}");

    // 1, 2, A(3)
}
