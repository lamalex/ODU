#![allow(clippy::len_without_is_empty)]
use crate::traits::Augment;
use num_traits::Num;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

/// Semantic alias for `Vector`. Represents single column vector of a 2D matrix.
pub type Col<T> = Vector<T>;
/// Semantic alias for `Vector`. Represents single row vector of a 2D matrix.
pub type Row<T> = Vector<T>;

#[derive(PartialEq, PartialOrd, Clone, Debug)]
/// A 1 dimensional, ordered collection of values which can represent
/// either a row or a column of a matrix. Not typically used by application
/// code. In most cases prefer [`Col`](type.Col.html) or [`Row`](type.Row.html).
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T>
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
    /// use launearalg::vector::Vector;
    ///
    /// let a = Vector::<u8>::new(10);
    /// ```
    /// Creates an n `Vector` A where n = 10
    /// with all entries A[i] == 0.
    pub fn new(len: usize) -> Self {
        assert!(len > 0);
        Vector {
            data: vec![T::zero(); len],
        }
    }

    /// Returns the number of elements in the vector, also referred to as its 'length'.
    ///
    /// # Example
    /// ```
    /// use launearalg::vector::Vector;
    ///
    /// let a = Vector::<u8>::new(5);
    /// assert_eq!(a.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Find position and value for maximum element of Vector
    ///
    /// # Panics
    /// Panics if there is no ordering between members of vector.
    /// Indicates invariant has been violated and somehow our Vector
    /// contains a NAN.
    ///
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let r = row![10, 55, 100, -200];
    /// let max = r.max_at();
    /// assert_eq!((2, 100), max);
    /// ```
    pub fn max_at(&self) -> (usize, T)
    where
        T: PartialOrd,
    {
        // 0 length rows are forbidden, and so unwrap() is used
        // rather than handling the optional.
        let max = self
            .data
            .iter()
            .enumerate()
            .max_by(|x, y| match x.1.partial_cmp(y.1) {
                Some(ord) => return ord,
                None => panic!(),
            })
            .unwrap();

        (max.0, *max.1)
    }

    /// Returns an Iterator<Item=T > over the elements of the Vector
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.data.iter()
    }

    fn apply_operation<F>(&self, rhs: T, mut op: F) -> Vector<T>
    where
        F: FnMut(T, T) -> T,
    {
        Vector {
            data: self.data.iter().map(|e| op(*e, rhs)).collect(),
        }
    }

    fn apply_operation_for_each<F>(&self, rhs: &Vector<T>, mut op: F) -> Vector<T>
    where
        F: FnMut(T, T) -> T,
    {
        assert_eq!(self.data.len(), rhs.data.len());
        Vector {
            data: self
                .data
                .iter()
                .enumerate()
                .map(|(i, x)| op(*x, rhs[i]))
                .collect(),
        }
    }
}

impl<T> Augment<&Vector<T>> for Vector<T>
where
    T: Copy,
{
    type Output = Self;
    /// # Example
    ///
    /// ```
    /// use launearalg::{row, vector::Row, traits::Augment, vector::Vector};
    ///
    /// let a = row![1, 2, 3];
    /// let b = row![9];
    ///
    /// let c = a.augment(&b);
    /// assert_eq!(row![1, 2, 3, 9], c);
    /// ```
    fn augment(&self, b: &Vector<T>) -> Vector<T> {
        Vector {
            data: self
                .data
                .iter()
                .copied()
                .chain(b.data.iter().copied())
                .collect(),
        }
    }
}

impl<T> Mul<T> for &Vector<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    /// Perform scalar multiplication for A * n
    /// where A is a Vector of <T>, and n is a scalar <T>.
    /// Note: n must be the right-hand side of the equation.
    /// **`n * A` will result in a compiler error**.
    ///
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let a = row![1, 2, 3, 4];
    ///
    /// let b = &a * 5;
    /// assert_eq!(row![5, 10, 15, 20], b);
    /// ```
    fn mul(self, rhs: T) -> Self::Output {
        self.apply_operation(rhs, Mul::mul)
    }
}

impl<T> Div<T> for &Vector<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    /// Perform scalar division for A / n
    /// where A is a vector of <T>, and n is a scalar <T>.
    /// Note: n must be the right-hand side of the equation.
    /// ``` n / A``` will result in a compiler error.
    ///
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let a = row![10, 20, 30, 40];
    ///
    /// let b = &a / 10;
    /// assert_eq!(row![1, 2, 3, 4], b);
    /// ```
    fn div(self, rhs: T) -> Self::Output {
        self.apply_operation(rhs, Div::div)
    }
}

impl<T> Add<&Vector<T>> for &Vector<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    /// Perform vector addition
    /// where A and B are a vectors of <T>.
    ///
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let a = row![10, 20, 30, 40];
    /// let b = row![10, 20, 30, 40];
    ///
    /// let c = &a + &b;
    /// assert_eq!(row![20, 40, 60, 80], c);
    /// ```
    fn add(self, rhs: &Vector<T>) -> Self::Output {
        self.apply_operation_for_each(rhs, Add::add)
    }
}

impl<T> Sub<&Vector<T>> for &Vector<T>
where
    T: Num + Copy,
{
    type Output = Vector<T>;
    /// Perform vector subtraction
    /// where A and B are a vectors of <T>.
    ///
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let a = row![10, 20, 30, 40];
    /// let b = row![10, 20, 30, 40];
    ///
    /// let c = &a - &b;
    /// assert_eq!(row![0, 0, 0, 0], c);
    /// ```
    fn sub(self, rhs: &Vector<T>) -> Self::Output {
        self.apply_operation_for_each(rhs, Sub::sub)
    }
}

impl<T> Index<usize> for Vector<T>
where
    T: Num + Copy,
{
    type Output = T;
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let row = row![1,2,3,4];
    ///
    /// // Accesses an element of `row`
    /// let el = row[0];
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector<T>
where
    T: Num + Copy,
{
    /// # Example
    /// ```
    /// use launearalg::{row, vector::Row};
    ///
    /// let mut row = row![1,2,3,4];
    ///
    /// // Accesses amutable element of `row`
    /// row[0] = 100;
    /// ```
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> From<Vec<T>> for Vector<T>
where
    T: Num + Copy,
{
    /// Creates a new `Vector` A from a `Vec<T>`
    ///
    /// # Example
    /// ```
    /// use launearalg::vector::Vector;
    ///
    /// let a = Vector::<u8>::from(vec![1, 2, 3, 4, 5, 6]);
    /// ```
    fn from(v: Vec<T>) -> Self {
        Vector::from(&v)
    }
}

impl<T> From<&Vec<T>> for Vector<T>
where
    T: Num + Copy,
{
    /// Creates a new `Vector` A from a `&Vec<T>`
    ///
    /// # Example
    /// ```
    /// use launearalg::vector::Vector;
    ///
    /// let v = vec![1, 2, 3, 4, 5, 6];
    /// let a = Vector::<u8>::from(&v);
    /// ```
    fn from(v: &Vec<T>) -> Self {
        assert!(!v.is_empty());
        Vector { data: v.to_vec() }
    }
}

/// Create a new row with convenient C-like array syntax
///
/// # Example
/// ```
/// use launearalg::{row, vector::Row};
///
/// let a = row![0.0, 0.1, 0.2];
/// let b = row![22.22; 100];
/// ```
#[macro_export]
macro_rules! row {
    ($($x:expr),* $(,)*) => {{
        Row::from(vec![$($x,)*])
    }};
    ($x:expr; $y:expr) => {{
        Row::from(vec![$x; $y])
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row_multiply() {
        let sut = Vector {
            data: vec![1, 2, 3],
        };

        assert_eq!(row![2, 4, 6], (&sut * 2));
    }

    #[test]
    fn test_max_at() {
        let sut = row![10, 100, 55, 24];
        assert_eq!((1, 100), sut.max_at());
    }

    #[test]
    fn test_max_at_all_same() {
        let sut = row![100, 100, 100, 100];
        assert_eq!((3, 100), sut.max_at());
    }
}
