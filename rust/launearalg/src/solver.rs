pub mod gauss {
    use crate::{matrix::Matrix, traits::PositionalMax, vector::Vector};
    use num_traits::Num;

    #[derive(Debug)]
    pub struct GaussianEliminationSolution<T>
    where
        T: Num,
    {
        weights: Vector<T>,
    }

    impl<T> std::fmt::Display for GaussianEliminationSolution<T>
    where
        T: Num + Copy + std::fmt::Display,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            let eqn = self
                .weights
                .iter()
                .enumerate()
                .map(|w| match w.0 {
                    0 => std::format!("{:15.4}", w.1),
                    1 => std::format!("{:8.4}x", w.1),
                    _ => std::format!("{:8.4}x^{}", w.1, w.0),
                })
                .fold_first(|a, b| std::format!("{} + {}", a, b));

            write!(f, "{}; global least squares approximation", eqn.unwrap())
        }
    }

    pub fn solve<T>(a: Matrix<T>) -> GaussianEliminationSolution<T>
    where
        T: PartialOrd + Num + Copy + num_traits::NumAssignOps,
    {
        assert_eq!(
            a.rows,
            a.cols - 1,
            "Gaussian elimination requires an augmented square matrix"
        );

        let mut a_prime = a.clone();

        for i in 0..a_prime.rows {
            // Swap
            let first_col_under_i = &a_prime[..][i][i..];
            if let Some(swap_value) = first_col_under_i.max_at() {
                a_prime = a_prime.swap_rows(i, swap_value.0 + i);
            }

            // Scale
            let first_entry = a_prime[i][i];
            // data_cols goes out of sync here. This whole she-bang needs rewritten to only
            // talk to a_prime, not with IndexMut which clearly needs unimplemented.
            a_prime[i] /= first_entry;

            // Eliminate
            eliminate(&mut a_prime, i);
        }

        // Backsolve
        backsolve(&mut a_prime);

        a_prime.sync();
        let weights = a_prime[..][a.cols - 1].clone();

        GaussianEliminationSolution { weights }
    }

    fn eliminate<T>(a: &mut Matrix<T>, start_col: usize)
    where
        T: PartialOrd + Num + Copy + num_traits::NumAssignOps,
    {
        for e in (start_col + 1)..a.rows {
            for j in start_col..a.cols {
                let s = a[e][start_col];
                let entry = s * a[start_col][j];
                a[e][j] -= entry;
            }
        }
    }

    fn backsolve<T>(a: &mut Matrix<T>)
    where
        T: PartialOrd + Num + Copy + num_traits::NumAssignOps,
    {
        // Backsolve
        let augment_i = a.cols - 1;
        for i in (1..augment_i).rev() {
            for j in (0..i).rev() {
                let s = a[j][i];
                let entry = s * a[i][i];
                a[j][i] -= entry;
                let entry = s * a[i][augment_i];
                a[j][augment_i] -= entry;
            }
        }
    }
}
