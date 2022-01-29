use type_reg::tagged::TypeReg;

fn main() {
    let mut type_reg = TypeReg::new();
    type_reg.register::<u32>();

    let deserializer = serde_yaml::Deserializer::from_str("u32: 1");
    let data_u32 = type_reg.deserialize_single(deserializer).unwrap();
    let data_u32 = data_u32.downcast_ref::<u32>().copied();

    println!("{data_u32:?}");

    // Some(1)
}
