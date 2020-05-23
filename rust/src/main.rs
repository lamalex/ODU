#[macro_use]
use matrixsolver::{mat, Matrix};

fn main() {
    println!("oh brother");
    let a = mat![
        [1, 0],
        [1, 1],
        [1, 2],
        [1, 3],
        [1, 4],
        [1, 5],
        [1, 6],
        [1, 7],
        [1, 8],
        [1, 9],
        [1, 10]
    ];
    println!("{} {}", a.rows, a.cols);
    let b = a.transpose();
    println!("{} {}", b.rows, b.cols);
    let c = b * a;
    println!("{} {}, {:?}", c.rows, c.cols, c);
}
