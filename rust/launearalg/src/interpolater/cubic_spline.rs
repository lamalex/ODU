use crate::{
    vector::Vector,
    matrix::Matrix,
    solver::gauss,
    traits::{Analyzer, Augment, Solution},
};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct CubicSplineInterpolationSolution {
    b: Vector<f64>,
    c: Vector<f64>,
    d: Vector<f64>,
    x: Vector<f64>,
}
impl Solution for CubicSplineInterpolationSolution {
    fn lhs(&self) -> &'static str {
        "S_"
    }
}

impl std::fmt::Display for CubicSplineInterpolationSolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.b.iter().enumerate().map(|(i, _x)| {
        writeln!(f, "{:9} = {:6} + {:20} + {:20} + {:20} on [{:5}, {:5}]; cubic spline", 
            format!("S_{}(x)", i), 
            format!("y_{}", i), 
            format!("{:.4}(x - {})", self.b[i], self.x[i]),
            format!("{:.4}(x - {})\u{00B2}", self.c[i], self.x[i]),
            format!("{:.4}(x - {})\u{00B3}", self.d[i], self.x[i]),
            self.x[i],
            self.x[i+1]
        )?;
        
        Ok(())
        }).collect()
    }
}
pub struct CubicSplineInterpolator {
    delta_x: Vec<f64>,
    delta_y: Vec<f64>,
    x_values: Vec<f64>,
}

impl CubicSplineInterpolator {
    pub fn new() -> CubicSplineInterpolator {
        CubicSplineInterpolator {
            delta_x: Vec::new(),
            delta_y: Vec::new(),
            x_values: Vec::new()
        }
    }
}

impl Analyzer for CubicSplineInterpolator {
    type Output = dyn Solution;

    fn analyze_piecewise(&mut self, points: Vec<(f64, f64)>) -> Option<Box<Self::Output>> {
        let d_x = points[1].0 - points[0].0;
        let d_y = points[1].1 - points[0].1;
        self.delta_x.push(d_x);
        self.delta_y.push(d_y);
        self.x_values.push(points[0].0);

        None
    }

    fn analyze_global(&mut self) -> Option<Box<Self::Output>> {
        let size = self.delta_x.len();

        let mut a = Matrix::<f64>::new(size, size);
        let mut b = Matrix::<f64>::new(size, 1);

        a[0][0] = 1.0;
        a[size - 1][size - 1] = 1.0;

        for i in 1..(size - 1) {
            a[i][i - 1] = self.delta_x[i - 1];
            a[i][i + 1] = self.delta_x[i];
            a[i][i] = 2.0 * (self.delta_x[i - 1] + self.delta_x[i]);
            b[i][0] =
                3.0 * self.delta_y[i] / self.delta_x[i] - self.delta_y[i - 1] / self.delta_x[i - 1];
        }

        let ab = a.augment(&b);
        let c_i = gauss::solve(ab);
        let mut b_i = vec![0.0; size - 1];
        let mut d_i = vec![0.0; size - 1];
        for i in 0..(size - 1) {
            b_i[i] = (self.delta_y[i] / self.delta_x[i])
                - (self.delta_x[i] / 3.0) * (2.0 * c_i[i] + c_i[i + 1]);
            d_i[i] = (c_i[i + 1] - c_i[i]) / (3.0 * self.delta_x[i]);
            
        }

        Some(Box::new(CubicSplineInterpolationSolution {
            b: Vector::from(b_i),
            c: Vector::from(c_i),
            d: Vector::from(d_i),
            x: Vector::from(self.x_values.clone())
        }))
    }
}