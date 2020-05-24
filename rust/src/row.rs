use num_traits::Num;
use std::ops::{Add, Deref, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Row<T> {
    data: Vec<T>,
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

impl<T> From<&[T]> for Row<T>
where
    T: Copy,
{
    fn from(s: &[T]) -> Self {
        Row { data: s.to_vec() }
    }
}

impl<T> Deref for Row<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
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
