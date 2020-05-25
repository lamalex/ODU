use matrixsolver::matrix::Matrix;
use matrixsolver::row::Row;
use matrixsolver::{mat, row};

fn main() {
    let mut c = mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("c: {:?}", c);
    let d = row![1, 2, 3, 4, 5];
    println!("d: {:?}", d);
    println!("d * 2: {:?}", &d * 2);
    println!("&c & 3: {:?}", &c * 3);
    println!("d: {:?}", d);
    println!("c: {:?}", c);
    c[0] = d;
    println!("c: {:?}", c);
}
