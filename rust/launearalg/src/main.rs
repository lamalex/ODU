use launearalg::{
    mat,
    matrix::Matrix,
    solver::gauss,
    traits::{Augment, Transpose},
};

fn main() {
    let x: Matrix<i32> = mat![[3, 3, 5], [3, 5, 9], [5, 9, 17]];
    let y: Matrix<i32> = mat![[5], [9], [17]];
    let x_t = x.transpose();
    let xtx = &x_t * &x;
    let xty = &x_t * &y;
    let xtx_xty = xtx.augment(&xty);

    println!("{:?}", gauss::solve(xtx_xty));
}
