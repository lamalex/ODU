use crate::traits::PositionalMax;

impl<T> PositionalMax<T> for [T]
where
    T: PartialOrd,
{
    fn max_at(&self) -> Option<(usize, &T)> {
        self.iter()
            .enumerate()
            .max_by(|x, y| x.1.partial_cmp(y.1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_at_for_slice() {
        let a = [1, 2, 3, 4, 5];
        let max = &a[..].max_at();
        assert_eq!(Some((4_usize, &5)), *max);
    }
}
