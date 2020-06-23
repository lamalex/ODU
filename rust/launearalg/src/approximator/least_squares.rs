use crate::traits::{Analyzer, Solution};
use crate::{
    matrix::Matrix,
    solver::gauss,
    traits::{Augment, Transpose},
    vector::Vector,
};
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

pub struct LeastSquaresApproximator {
    x: Vec<Vec<f64>>,
    y: Vec<Vec<f64>>,
}

impl LeastSquaresApproximator {
    pub fn new() -> LeastSquaresApproximator {
        LeastSquaresApproximator {
            x: vec![],
            y: vec![],
        }
    }
}
impl Analyzer for LeastSquaresApproximator {
    type Output = dyn Solution;

    fn analyze_piecewise(&mut self, points: Vec<(f64, f64)>) -> Option<Box<Self::Output>> {
        self.x.push(vec![1.0, points[0].0]);
        self.y.push(vec![points[0].1]);
        None
    }

    fn analyze_global(&mut self) -> Option<Box<Self::Output>> {
        let core_x = Matrix::from(self.x.clone());
        let core_y = Matrix::from(self.y.clone());

        let core_xt = core_x.transpose();
        let core_xtx = &core_xt * &core_x;
        let core_xty = &core_xt * &core_y;
        let core_xtxxty = core_xtx.augment(&core_xty);

        let weights = gauss::solve(core_xtxxty);
        Some(Box::new(LeastSquaresApproximationSolution { weights }))
    }
}
