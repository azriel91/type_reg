use crate::de::TypeSeed;

/// A seed factory for a fixed set of types.
///
/// This simply creates a new `TypeSeed` with the given tag.
struct TypeSeedFactory;

impl<'de> SeedFactory<'de, &'de str> for TypeSeedFactory {
    type Value = Box<TypeId>;
    type Seed = TypeSeed<'de>;

    fn seed<E>(self, tag: &'de str) -> Result<Self::Seed, E>
    where
        E: serde::de::Error,
    {
        Ok(TypeSeed::new(tag))
    }
}
