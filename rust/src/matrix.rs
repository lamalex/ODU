use num_traits::Num;
use std::ops::{Index, Mul};

use crate::{row, vector::Col, vector::Row, vector::Vector};

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
    /// use matrixsolver::matrix::Matrix;
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

    /// Create a new `Matrix` B, which is the transpose of `Matrix` A,
    /// that is to say `A[i][j] == B[j][i]`.
    pub fn transpose(&self) -> Self {
        let a_t = vec_transpose(&self.data_rows);
        let mut b = Matrix::from(a_t);
        b.data_cols = self.data_rows.clone();

        b
    }
}

fn vec_transpose<T>(data: &Vec<Vector<T>>) -> Vec<Vec<T>>
where
    T: Num + Copy,
{
    let mut b = vec![vec![T::zero(); data.len()]; data[0].len()];
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            b[j][i] = data[i][j];
        }
    }
    b
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
    /// use matrixsolver::matrix::Matrix;
    ///
    /// let a = Matrix::<u8>::from(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    /// ```
    /// Creates a new `Matrix` with 2 rows, each with 3 columns.
    fn from(v: Vec<Vec<T>>) -> Self {
        let mut b = Matrix {
            rows: v.len(),
            cols: v[0].len(),
            data_rows: v.iter().map(Vector::from).collect(),
            data_cols: vec![row![T::zero(); v.len()]; v[0].len()],
        };

        b.data_cols = vec_transpose(&b.data_rows)
            .iter()
            .map(Vector::from)
            .collect();

        b
    }
}

/// # Example
/// ```
/// use matrixsolver::{mat, matrix::Matrix};
///
/// let matrix = mat![[1,2],[3,4]];
///
/// // Accesses a single row of `matrix`
/// let row = &matrix[0];
/// ```
impl<T> Index<usize> for Matrix<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data_rows[index]
    }
}

/// # Example
/// ```
/// use matrixsolver::{mat, row, matrix::Matrix, vector::Row};
///
/// let expected = vec![row![1, 3], row![2, 4]];
///
/// let matrix = mat![[1,2],[3,4]];
///
/// // Accesses columns of `matrix`
/// let cols = &matrix[..];

/// let first_col = &matrix[..][0];
/// ```
impl<T> Index<std::ops::RangeFull> for Matrix<T>
where
    T: Num + Copy,
{
    type Output = Vec<Col<T>>;
    fn index(&self, _index: std::ops::RangeFull) -> &Self::Output {
        &self.data_cols
    }
}

/// Perform matrix multiplication for `matrix` A x B = C
/// where A is of size m x n, B is of size n x p, and C is of size m x p.
///
/// # Panics
/// Panics when B is not of size n x p for A of size m x n.
///
/// # Example
/// ```
/// use matrixsolver::{mat, matrix::Matrix};
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
impl<T> Mul<&Matrix<T>> for &Matrix<T>
where
    T: Num + Copy,
{
    type Output = Matrix<T>;
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

/// Perform scalar multiplication for A * n
/// where A a matrix of <T>, and n is a scalar <T>.
/// Note: n must be the right-hand side of the equation.
/// ``` n * A``` will result in a compiler error.
///
/// # Example
/// ```
/// use matrixsolver::{mat, matrix::Matrix};
///
/// let a = mat![
///     [1, 2],
///     [3, 4]
/// ];
///
/// let b = &a * 5;
/// assert_eq!(mat![[5, 10], [15, 20]], b);
/// ```
impl<T> Mul<T> for &Matrix<T>
where
    T: Num + Copy,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Matrix<T> {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data_rows: self.data_rows.iter().map(|x| x * rhs).collect(),
            data_cols: self.data_cols.iter().map(|x| x * rhs).collect(),
        }
    }
}

/// Create a new matrix with convenient C-like 2D array syntax.None
///
/// # Example
/// ```
/// use matrixsolver::{mat, matrix::Matrix};
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
}
