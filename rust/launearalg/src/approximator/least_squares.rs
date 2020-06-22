use crate::traits::Solution;
use crate::vector::Vector;
use std::fmt;

#[derive(Debug)]
pub struct LeastSquaresApproximationSolution {
    pub weights: Vector<f64>,
}

impl Solution for LeastSquaresApproximationSolution {
    fn lhs(&self) -> &'static str {
        "φ\u{0302}"
    }
}

impl fmt::Display for LeastSquaresApproximationSolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        let eqn = self
            .weights
            .iter()
            .enumerate()
            .map(|w| match w.0 {
                0 => std::format!("{:15.4}", w.1),
                1 => std::format!("{:8.4}x", w.1),
                _ => std::format!("{:8.4}x^{}", w.1, w.0),
            })
            // requires nightly.
            //.fold_first(|a, b| std::format!("{} + {}", a, b))
            //.unwrap()
            .collect::<Vec<String>>()
            .join(" + ");

        write!(f, "{}; global least squares approximation", eqn)
    }
}

pub struct LeastSquaresApproximator;
