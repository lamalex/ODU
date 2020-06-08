use launearalg::{
    mat,
    matrix::Matrix,
    traits::{Augment, Transpose},
};

fn main() {
    let x: Matrix<f64> = mat![[1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 2.0, 4.0]];
    let y: Matrix<f64> = mat![[0.0], [1.0], [4.0]];
    let x_t = x.transpose();
    let xtx = &x_t * &x;
    let xty = &x_t * &y;
    let mut xtx_xty = xtx.augment(&xty);
    let row_with_max = xtx_xty[..][0].max_at();
    println!("{:?}", xtx_xty);
    xtx_xty = xtx_xty.swap_rows(0, row_with_max.0);
    println!("{:?}", xtx_xty);
}
