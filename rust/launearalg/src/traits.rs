/// Appends b onto self
pub trait Augment<B = Self> {
    type Output;
    fn augment(&self, b: B) -> Self::Output;
}

/// Creates a new `Output`, we will call B such that for self, A, B[i][j] = A[j][i].
pub trait Transpose {
    type Output;
    fn transpose(&self) -> Self::Output;
}

pub trait PositionalMax<T>
where
    T: PartialOrd,
{
    fn max_at(&self) -> Option<(usize, &T)>;
}

pub trait Solution: std::fmt::Display {
    fn lhs(&self) -> &'static str;
}
