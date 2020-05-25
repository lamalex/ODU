use num_traits::Num;
use std::ops::{Add, Deref, DerefMut, Mul};

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
}

impl<T> Mul<T> for &Row<T>
where
    T: Num + Copy,
{
    type Output = Row<T>;
    fn mul(self, rhs: T) -> Row<T> {
        Row {
            data: self.data.iter().map(|x| *x * rhs).collect(),
        }
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

impl<T> Deref for Row<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Row<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
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

        assert_eq!(vec![2, 4, 6], *(&sut * 2));
    }
}
