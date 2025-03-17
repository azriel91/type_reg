use type_reg::untagged::TypeReg;

fn main() {
    let mut type_reg = TypeReg::<String>::new();
    type_reg.register::<u32>(String::from("one"));

    let deserializer = serde_yaml_ng::Deserializer::from_str("one: 1");
    let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    let data_u32 = data_u32.downcast_ref::<u32>().copied();

    println!("{data_u32:?}");

    // Some(1)
}
