use crate::interpolater::linear_piecewise::LinearPiecewiseInterpolationSolution;
use num_traits::Num;

pub trait InterpolationSolution {}

pub trait Interpolate<T>
where
    T: Num,
{
    fn interpolate(points: Vec<(T, T)>) -> Option<LinearPiecewiseInterpolationSolution<T>>;
}
