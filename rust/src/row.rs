use num_traits::Num;
use std::ops::{Deref, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Row<T> {
    data: Vec<T>,
}

impl<T> Deref for Row<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Mul<T> for Row<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Row {
            data: self.data.iter().map(|x| *x * rhs).collect(),
        }
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

        assert_eq!(vec![2, 4, 6], *(sut * 2));
    }
}
