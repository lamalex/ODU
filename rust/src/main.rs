use matrixsolver::{mat, matrix::Matrix};

fn main() {
    let x = mat![
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

    let y = mat![
        [0],
        [1],
        [4],
        [9],
        [16],
        [25],
        [36],
        [49],
        [64],
        [81],
        [100]
    ];

    let x_t = x.transpose();
    let xtx = &x_t * &x;
    let xty = &x_t * &y;

    println!("xtx: {:?}", xtx);
    println!("xty: {:?}", xty);
}
