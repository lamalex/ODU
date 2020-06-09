use launearalg::{
    mat,
    matrix::Matrix,
    traits::{Augment, PositionalMax, Transpose},
};

fn main() {
    let x: Matrix<f64> = mat![[1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 2.0, 4.0]];
    let y: Matrix<f64> = mat![[0.0], [1.0], [4.0]];
    let x_t = x.transpose();
    let xtx = &x_t * &x;
    let xty = &x_t * &y;
    let mut xtx_xty = xtx.augment(&xty);

    for i in 0..xtx_xty.rows {
        // Swap
        let first_col_under_i = &xtx_xty[..][i][i..];
        if let Some(swap_value) = first_col_under_i.max_at() {
            xtx_xty = xtx_xty.swap_rows(i, swap_value.0 + i);
        }

        // Scale
        let first_entry = xtx_xty[i][i];
        // data_cols goes out of sync here. This whole she-bang needs rewritten to only
        // talk to xtx_xty, not with IndexMut which clearly needs unimplemented.
        xtx_xty[i] /= first_entry;

        // Eliminate
        let start_col = i;
        for e in (i + 1)..xtx_xty.rows {
            for j in start_col..xtx_xty[..].len() {
                let s = xtx_xty[e][start_col];
                xtx_xty[e][j] -= s * xtx_xty[i][j];
            }
        }
    }

    // Backsolve
    let augment_i = xtx_xty[0].len() - 1;
    for i in (1..augment_i).rev() {
        for j in (0..i).rev() {
            let s = xtx_xty[j][i];
            xtx_xty[j][i] -= s * xtx_xty[i][i];
            xtx_xty[j][augment_i] -= s * xtx_xty[i][augment_i];
        }
    }

    println!("{:?}", xtx_xty);
}
