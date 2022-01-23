/// A `DeserializeSeed` implementation for a fixed set of types.
///
/// Decides which type should be deserialized by using a simple match statement
/// on the tag.
pub struct TypeSeed<'de> {
    tag: &'de str,
}

impl<'de> TypeSeed<'de> {
    fn new(tag: &'de str) -> Self {
        TypeSeed { tag }
    }
}

impl<'de> serde::de::DeserializeSeed<'de> for TypeSeed<'de> {
    type Value = Box<TypeId>;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match self.tag {
            "Message" => Ok(Box::new(Message::deserialize(deserializer)?)),
            "i64" => Ok(Box::new(i64::deserialize(deserializer)?)),
            tag => Err(serde::de::Error::unknown_variant(tag, &["Message"])),
        }
    }
}
