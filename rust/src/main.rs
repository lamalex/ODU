use matrixsolver::{mat, matrix::Matrix, Augment};

fn main() {
    let x = mat![[1, 0, 0], [1, 1, 1], [1, 2, 4]];
    let y = mat![[0], [1], [4]];
    let x_t = x.transpose();
    let xtx = &x_t * &x;
    let xty = &x_t * &y;
    let xtx_xty = xtx.augment(&xty);
    println!("{:?}", xtx_xty);
}
