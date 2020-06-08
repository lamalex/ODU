pub trait Augment<B = Self> {
    /// Appends b onto self
    type Output;
    fn augment(&self, b: B) -> Self::Output;
}

pub trait Transpose {
    type Output;
    fn transpose(&self) -> Self::Output;
}
