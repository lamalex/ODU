use matrixsolver::matrix::Matrix;
use matrixsolver::row::Row;
use matrixsolver::{mat, row};

fn main() {
    let mut a = mat![
        [1.0, 2.0, 3.0, 4.0, 5.0],
        [10.0, 10.0, 10.0, 10.0, 10.0],
        [5.0, 5.0, 5.0, 5.0, 5.0]
    ];
    println!("{:?}", a);
    a[1] = &a[1] - &(&a[0] * (a[1][0] / a[0][0]));
    println!("{:?}", a);
}
