/// Trait to downcast from a boxed DataType to the concrete type.
pub trait BoxDataTypeDowncast<T> {
    fn downcast_ref(&self) -> Option<&T>;

    fn downcast_mut(&mut self) -> Option<&mut T>;
}
