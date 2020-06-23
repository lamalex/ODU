use crate::traits::{Analyzer, Interpolate, Solution};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct LinearPiecewiseInterpolationSolution {
    c0: f64,
    c1: f64,
}
impl Solution for LinearPiecewiseInterpolationSolution {
    fn lhs(&self) -> &'static str {
        "y_"
    }
}

impl std::fmt::Display for LinearPiecewiseInterpolationSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:15.4} + {:8.4}x; interpolation", self.c0, self.c1)
    }
}
pub struct LinearPiecewiseInterpolater;
impl LinearPiecewiseInterpolater {
    pub fn new() -> LinearPiecewiseInterpolater {
        LinearPiecewiseInterpolater {}
    }
}

impl Interpolate for LinearPiecewiseInterpolater {
    type Output = LinearPiecewiseInterpolationSolution;
    fn interpolate(points: Vec<(f64, f64)>) -> Option<Self::Output> {
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

impl Analyzer for LinearPiecewiseInterpolater {
    type Output = dyn Solution;

    fn analyze_piecewise(&mut self, points: Vec<(f64, f64)>) -> Option<Box<Self::Output>> {
        match LinearPiecewiseInterpolater::interpolate(points) {
            Some(sol) => Some(Box::new(sol)),
            None => None,
        }
    }

    fn analyze_global(&mut self) -> Option<Box<Self::Output>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpolate_2points() {
        let p1 = (2.0, 1.0);
        let p2 = (4.0, 2.0);

        let interp_res = LinearPiecewiseInterpolater::interpolate(vec![p1, p2]);
        assert_eq!(
            LinearPiecewiseInterpolationSolution { c0: 0.0, c1: 0.5 },
            interp_res.unwrap()
        );
    }
}
