use num_traits::Num;

pub trait InterpolationSolution {}

pub trait Interpolate<T>
where
    T: Num,
{
    type Output;
    fn interpolate(points: Vec<(T, T)>) -> Option<Self::Output>;
}
