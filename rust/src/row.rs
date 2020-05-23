use num_traits::Num;
use std::ops::{Deref, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Row<'a, T> {
    pub data: &'a [T],
}

impl<'a, T> Deref for Row<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> Mul<T> for Row<'a, T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn mul(self, _rhs: T) -> Self {
        let result = self.data.clone();
        Row { data: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row_multiply() {
        let sut = Row { data: &[1, 2, 3] };

        assert_eq!([2, 4, 6], *(sut * 2));
    }
}
