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

pub trait Solution: std::fmt::Display
where
    Self: Sized,
{
    fn lhs(&self) -> &'static str;
}

pub trait Analyzer
where
    Self::Output: Solution,
{
    type Output;
    fn analyze_piecewise(&self, points: Vec<(f64, f64)>) -> Option<Box<Self::Output>>;
    fn analyze_global(&self, x_data: Vec<f64>, y_data: Vec<f64>) -> Option<Box<Self::Output>>;
}

pub trait Approximate
where
    Self::Output: Solution,
{
    type Output;
    fn approximate() -> Option<Self::Output>;
}

pub trait Interpolate
where
    Self::Output: Solution,
{
    type Output;
    fn interpolate(points: Vec<(f64, f64)>) -> Option<Self::Output>;
}
