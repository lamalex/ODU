use num_traits::Num;
use std::ops::{Add, Div, Index, IndexMut, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Row<T> {
    data: Vec<T>,
}

impl<T> Row<T>
where
    T: Num + Copy,
{
    pub fn new(len: usize) -> Self {
        assert!(len > 0);
        Row {
            data: vec![T::zero(); len],
        }
    }

    fn apply_operation<F>(&self, rhs: T, mut op: F) -> Row<T>
    where
        F: FnMut(T, T) -> T,
    {
        Row {
            data: self.data.iter().map(|e| op(*e, rhs)).collect(),
        }
    }
}

impl<T> Mul<T> for &Row<T>
where
    T: Num + Copy,
{
    type Output = Row<T>;
    fn mul(self, rhs: T) -> Row<T> {
        self.apply_operation(rhs, Mul::mul)
    }
}

impl<T> Div<T> for &Row<T>
where
    T: Num + Copy,
{
    type Output = Row<T>;
    fn div(self, rhs: T) -> Row<T> {
        self.apply_operation(rhs, Div::div)
    }
}

impl<T> Add<&Row<T>> for &Row<T>
where
    T: Num + Copy,
{
    type Output = Row<T>;
    fn add(self, rhs: &Row<T>) -> Row<T> {
        assert_eq!(self.data.len(), rhs.data.len());
        Row {
            data: self
                .data
                .iter()
                .enumerate()
                .map(|(i, x)| *x + rhs[i])
                .collect(),
        }
    }
}

impl<T> From<Vec<T>> for Row<T>
where
    T: Num + Copy,
{
    fn from(v: Vec<T>) -> Self {
        let mut a = Row::<T>::new(v.len());
        a.data = v;
        a
    }
}

impl<T> Index<usize> for Row<T>
where
    T: Num + Copy,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Row<T>
where
    T: Num + Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> From<&Vec<T>> for Row<T>
where
    T: Num + Copy,
{
    fn from(v: &Vec<T>) -> Self {
        let mut a = Row::<T>::new(v.len());
        a.data = v.clone();
        a
    }
}

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
        let sut = Row {
            data: vec![1, 2, 3],
        };

        assert_eq!(row![2, 4, 6], (&sut * 2));
    }
}
