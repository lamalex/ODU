pub mod matrix {
    use num_traits::Num;
    use std::ops::{Index, IndexMut};

    #[derive(Debug, PartialEq)]
    pub struct Matrix<T> {
        pub rows: usize,
        pub cols: usize,
        data: Vec<T>,
    }

    impl<T> Matrix<T>
    where
        T: Num + Copy,
    {
        pub fn new(rows: usize, cols: usize) -> Self {
            assert!(rows > 0 && cols > 0);
            Matrix {
                rows: rows,
                cols: cols,
                data: vec![T::zero(); rows * cols],
            }
        }

        pub fn from(v: Vec<Vec<T>>) -> Self {
            let mut a = Matrix::<T>::new(v.len(), v[0].len());
            for (i, e) in v.iter().flatten().enumerate() {
                a.data[i] = *e;
            }
            a
        }

        pub fn transpose(self) -> Self {
            let mut a = Matrix::<T>::new(self.cols, self.rows);
            for i in 0..self.rows {
                for j in 0..self.cols {
                    a[j][i] = self[i][j];
                }
            }
            a
        }
    }

    impl<T> Index<usize> for Matrix<T> {
        type Output = [T];
        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index * self.cols..(index + 1) * self.cols]
        }
    }

    impl<T> IndexMut<usize> for Matrix<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.data[index * self.cols..(index + 1) * self.cols]
        }
    }

    #[macro_export]
    macro_rules! mat {
        ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
            crate::matrix::Matrix::from(vec![$([$($x,)*].to_vec(),)*])
        }}
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        #[should_panic]
        fn test_panic_on_0_size() {
            let _a = Matrix::<u8>::new(0, 0);
            let _b = Matrix::<u8>::new(1, 0);
            let _a = Matrix::<u8>::new(0, 1);
        }

        #[test]
        fn test_create_matrix_macro() {
            let sut = mat![[1, 2], [3, 4]];
            assert_eq!(sut.data, [1, 2, 3, 4]);
        }

        #[test]
        fn test_index_matrix() {
            let sut = mat![[1, 2], [3, 4]];
            assert_eq!(sut[0][0], 1);
            assert_eq!(sut[0][1], 2);
            assert_eq!(sut[1][0], 3);
            assert_eq!(sut[1][1], 4);
        }

        #[test]
        #[should_panic]
        fn test_index_out_of_bounds_panic() {
            let sut = mat![[1, 2], [3, 4]];
            let _bad_access = sut[5][5];
        }

        #[test]
        fn test_matrix_equality() {
            let sut = mat![[1, 2], [3, 4]];
            assert_eq!(sut, mat![[1, 2], [3, 4]]);
        }

        #[test]
        fn test_matrix_inequality() {
            let sut = mat![[1, 2], [3, 4]];
            assert_ne!(sut, mat![[0, 0], [0, 0]]);
            assert_ne!(sut, mat![[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        }

        #[test]
        fn test_transpose_matrix() {
            let sut = mat![[1, 2, 3], [4, 5, 6]];
            let tx = sut.transpose();
            assert_eq!(tx, mat![[1, 4], [2, 5], [3, 6]]);
        }
    }
}
