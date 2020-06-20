use crate::traits::{Interpolate, InterpolationSolution};
use num_traits::Num;

#[derive(Debug)]
pub struct LinearPiecewiseInterpolationSolution<T>
where
    T: Num,
{
    c0: T,
    c1: T,
}
impl<T> InterpolationSolution for LinearPiecewiseInterpolationSolution<T> where T: Num {}

impl<T> std::fmt::Display for LinearPiecewiseInterpolationSolution<T>
where
    T: Num + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:15.4} + {:8.4}x; interpolation", self.c0, self.c1)
    }
}
pub struct LinearPiecewiseInterpolater;

impl<T> Interpolate<T> for LinearPiecewiseInterpolater
where
    T: Num + Copy,
{
    fn interpolate(points: Vec<(T, T)>) -> Option<LinearPiecewiseInterpolationSolution<T>> {
        if points.len() != 2 {
            return None;
        }

        if points[0].0 == points[1].0 {
            return None;
        }

        let p1 = points[0];
        let p2 = points[1];

        let c1 = (p2.1 - p1.1) / (p2.0 - p1.0);
        let c0 = p1.1 - c1 * p1.0;

        Some(LinearPiecewiseInterpolationSolution { c0, c1 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_2points() {
        let p1 = (2.0, 1.0);
        let p2 = (4.0, 2.0);

        //let interp_res = LinearPiecewiseInterpolater::interpolate(vec![p1, p2]);
        //assert_eq!((0.0, 0.5), interp_res.unwrap());
    }
}
