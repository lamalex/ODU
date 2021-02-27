use crate::traits::Transpose;
use num_traits::Num;

impl<T> Transpose for Vec<Vec<T>>
where
    T: Num + Copy,
{
    type Output = Self;
    fn transpose(&self) -> Self::Output
    where
        T: Copy,
    {
        let mut b = vec![vec![T::zero(); self.len()]; self[0].len()];
        for (i, vec) in self.iter().enumerate() {
            for (j, el) in vec.iter().enumerate() {
                b[j][i] = *el;
            }
        }
        b
    }
}
