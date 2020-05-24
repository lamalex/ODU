use matrixsolver::mat;
use matrixsolver::matrix::Matrix;
use matrixsolver::row::Row;

fn main() {
    println!("oh brother");
    let a = mat![[1, 2], [3, 4]];
    let _b = &a[0];
    println!("{:?}", a);
    let c = Row::from(&a[0]);
    let _d = &c * 2;
}
