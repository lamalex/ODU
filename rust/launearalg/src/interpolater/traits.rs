use crate::traits::Solution;
use num_traits::Num;

pub trait Interpolate<T>
where
    T: Num,
    Self::Output: Solution,
{
    type Output;
    fn interpolate(points: Vec<(T, T)>) -> Option<Self::Output>;
}
