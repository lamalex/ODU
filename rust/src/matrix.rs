use num_traits::Num;
use std::ops::{Index, IndexMut, Mul};

use crate::{
    row,
    traits::{Augment, Transpose},
    vector::Col,
    vector::Row,
    vector::Vector,
};

#[derive(Debug, PartialEq, Clone)]
/// High level struct describing a 2D matrix
pub struct Matrix<T> {
    /// Number of rows for instance of Matrix
    pub rows: usize,
    /// Number of columns for instance of Matrix
    pub cols: usize,
    data_rows: Vec<Row<T>>,
    data_cols: Vec<Col<T>>,
}

impl<T> Matrix<T>
where
    T: Num + Copy,
{
    /// Create a new zeroed `Matrix` of size rows x cols
    ///
    /// See also the [`From` impls](#impl-From<Vec<Vec<T>>>), or [`mat!`](../macro.mat.html) macro for creating
    /// an initialized `Matrix`.
    ///
    /// # Panics
    /// Panics if rows or cols ≦ 0
    ///
    /// # Example
    /// ```
    /// use launearalg::matrix::Matrix;
    ///
    /// let a = Matrix::<u8>::new(10, 10);
    /// ```
    /// Creates a `Matrix` A with 10 rows by 10 columns
    /// with all entries A[i][j] == 0.
    pub fn new(rows: usize, cols: usize) -> Self {
        assert!(rows > 0 && cols > 0);
        Matrix {
            rows,
            cols,
            data_rows: vec![row![T::zero(); cols]; rows],
            data_cols: vec![row![T::zero(); rows]; cols],
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Vector<T>> {
        self.data_rows.iter()
    }

    /// Returns a new matrix whose rows with given indices are swapped.
    ///
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix};
    ///
    /// let a = mat![[1,2,3], [3,2,1]];
    /// assert_eq!(mat![[3, 2, 1], [1, 2, 3]], a.swap_rows(0, 1));
    /// ```
    pub fn swap_rows(&self, r1: usize, r2: usize) -> Matrix<T>
    where
        T: Copy,
    {
        let mut data_rows = self.data_rows.clone();
        Matrix::swapsies(&mut data_rows, r1, r2);
        let data_cols = data_rows.transpose();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data_rows,
            data_cols,
        }
    }

    // The great thing about private members is you can name them whatever you want
    // and as long as the next person who comes along can understand what you wrote,
    // you're squared away!
    fn swapsies(data: &mut Vec<Vector<T>>, r1: usize, r2: usize)
    where
        T: Copy,
    {
        let tmp = data[r1].clone();
        data[r1] = data[r2].clone();
        data[r2] = tmp;
    }
}

impl<T> Transpose for Matrix<T>
where
    T: Clone,
{
    /// Create a new `Matrix` B, which is the transpose of `Matrix` A,
    /// that is to say `A[i][j] == B[j][i]`.
    ///
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix, traits::Transpose};
    ///
    /// let a = mat![[1,2],[3,4]];
    /// let a_t = a.transpose();
    ///
    /// assert_eq!(mat![[1,3],[2,4]], a_t);
    /// ```
    type Output = Matrix<T>;
    fn transpose(&self) -> Self::Output {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data_rows: self.data_cols.clone(),
            data_cols: self.data_rows.clone(),
        }
    }
}

impl<T> Augment<&Matrix<T>> for Matrix<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn augment(&self, b: &Matrix<T>) -> Matrix<T> {
        assert!(self.rows == b.rows);
        let augmented = self
            .data_rows
            .iter()
            .zip(b.data_rows.iter())
            .map(|(ra, rb)| ra.augment(rb))
            .collect::<Vec<Vector<T>>>();

        let aug_t = augmented.transpose();

        Matrix {
            rows: self.rows,
            cols: self.cols + b.cols,
            data_rows: augmented,
            data_cols: aug_t,
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Matrix<T>
where
    T: Num + Copy,
{
    /// Creates a new `Matrix` A from a `Vec<Vec<T>>` where each Vec<T> represents
    /// a row of the young `Matrix`.
    ///
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix};
    ///
    /// let a = Matrix::<u8>::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// assert_eq!(mat![[1, 2, 3], [4, 5, 6]], a);
    /// ```
    fn from(v: Vec<Vec<T>>) -> Self {
        Matrix {
            rows: v.len(),
            cols: v[0].len(),
            data_rows: v.iter().map(Vector::from).collect(),
            data_cols: v.transpose().iter().map(Vector::from).collect(),
        }
    }
}

impl<T> Index<usize> for Matrix<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix};
    ///
    /// let matrix = mat![[1,2],[3,4]];
    ///
    /// // Accesses a single row of `matrix`
    /// let row = &matrix[0];
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.data_rows[index]
    }
}

impl<T> IndexMut<usize> for Matrix<T>
where
    T: Num + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data_rows[index]
    }
}

impl<T> Index<std::ops::RangeFull> for Matrix<T>
where
    T: Num + Copy,
{
    type Output = [Col<T>];
    /// # Example
    /// ```
    /// use launearalg::{mat, row, matrix::Matrix, vector::Row};
    ///
    /// let expected = vec![row![1, 3], row![2, 4]];
    ///
    /// let matrix = mat![[1,2],[3,4]];
    ///
    /// // Accesses columns of `matrix`
    /// let cols = &matrix[..];
    ///
    /// let first_col = &matrix[..][0];
    /// ```
    fn index(&self, _index: std::ops::RangeFull) -> &Self::Output {
        &self.data_cols[..]
    }
}

impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Num + Copy,
{
    type Output = Matrix<T>;
    /// Perform matrix multiplication for `matrix` A x B = C
    /// where A is of size m x n, B is of size n x p, and C is of size m x p.
    ///
    /// # Panics
    /// Panics when B is not of size n x p for A of size m x n.
    ///
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix};
    ///
    /// let a = mat![
    ///     [1, 2, 3],
    ///     [1, 2, 3]
    /// ];
    /// let b = mat![
    ///     [4, 5],
    ///     [6, 7],
    ///     [8, 9]
    /// ];
    ///
    /// let c = &a * &b;
    /// assert_eq!(mat![[40, 46], [40, 46]], c);
    /// ```
    fn mul(self, rhs: &Matrix<T>) -> Matrix<T> {
        let mut result = Matrix::new(self.rows, rhs.cols);

        for i in 0..result.rows {
            for j in 0..result.cols {
                for k in 0..self.cols {
                    let product = result[i][j] + self[i][k] * rhs[k][j];
                    result.data_rows[i][j] = product;
                    result.data_cols[j][i] = product;
                }
            }
        }
        result
    }
}

impl<T> Mul<T> for &Matrix<T>
where
    T: Num + Copy,
{
    type Output = Matrix<T>;
    /// Perform scalar multiplication for A * n
    /// where A a matrix of <T>, and n is a scalar <T>.
    /// Note: n must be the right-hand side of the equation.
    /// ``` n * A``` will result in a compiler error.
    ///
    /// # Example
    /// ```
    /// use launearalg::{mat, matrix::Matrix};
    ///
    /// let a = mat![
    ///     [1, 2],
    ///     [3, 4]
    /// ];
    ///
    /// let b = &a * 5;
    /// assert_eq!(mat![[5, 10], [15, 20]], b);
    /// ```
    fn mul(self, rhs: T) -> Matrix<T> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data_rows: self.data_rows.iter().map(|x| x * rhs).collect(),
            data_cols: self.data_cols.iter().map(|x| x * rhs).collect(),
        }
    }
}

/// Create a new matrix with convenient C-like 2D array syntax
///
/// # Example
/// ```
/// use launearalg::{mat, matrix::Matrix};
///
/// let a = mat![
///     [0.0, 0.1, 0.2],
///     [0.3, 0.4, 0.5],
///     [0.6, 0.7, 0.8]
/// ];
/// ```
#[macro_export]
macro_rules! mat {
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        Matrix::from(vec![$([$($x,)*].to_vec(),)*])
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
    fn test_index_matrix() {
        let sut = mat![[1, 2], [3, 4]];
        assert_eq!(sut[0][0], 1);
        assert_eq!(sut[0][1], 2);
        assert_eq!(sut[1][0], 3);
        assert_eq!(sut[1][1], 4);
        assert_eq!(sut[0], row![1, 2]);
    }

    #[test]
    fn test_index_column() {
        let sut = mat![[1, 2], [3, 4]];
        assert_eq!(row![1, 3], sut[..][0]);
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
    #[test]
    fn test_transpose_symmetric_matrix() {
        let sut = mat![[1, 2], [2, 1]];
        assert_eq!(sut.transpose(), mat![[1, 2], [2, 1]]);
    }

    #[test]
    fn test_matrix_multiply() {
        let sut = mat![
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
            [1, 10],
        ];

        let sut_t = sut.transpose();

        assert_eq!(&sut_t * &sut, mat![[11, 55], [55, 385]]);
    }

    #[test]
    #[should_panic]
    fn test_matrix_multiply_panic_missize() {
        let _c = &mat![[1, 2, 3], [4, 5, 6]] * &mat![[1, 2], [3, 4]];
    }

    #[test]
    fn test_matrix_scalar_multiplication() {
        let sut = mat![[2, 4], [6, 8]];
        assert_eq!(&sut * 2, mat![[4, 8], [12, 16]]);
    }

    #[test]
    fn test_matrix_scalar_multiplication_float() {
        let sut = mat![[2.0, 4.0], [6.0, 8.0]];
        assert_eq!(&sut * 1.5, mat![[3.0, 6.0], [9.0, 12.0]]);
    }

    #[test]
    fn test_augment_compatible() {
        let a = mat![[3, 3, 5], [3, 5, 9], [5, 9, 17]];
        let b = mat![[5], [9], [17]];
        let expected = mat![[3, 3, 5, 5], [3, 5, 9, 9], [5, 9, 17, 17]];
        let c = a.augment(&b);
        assert_eq!(expected, c);
    }

    #[test]
    #[should_panic]
    fn test_augment_incompatible() {
        let a = mat![[3, 3, 5], [3, 5, 9]];
        let b = mat![[5], [9], [17]];
        let _c = a.augment(&b);
    }

    #[test]
    fn test_swap_rows() {
        let sut = mat![[1, 2, 3], [3, 2, 1]];
        let expected = mat![[3, 2, 1], [1, 2, 3]];
        assert_eq!(expected, sut.swap_rows(0, 1));
    }
}
