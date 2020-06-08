use crate::{row, traits::Transpose, vector::Row, vector::Vector};
use num_traits::Num;

impl<T> Transpose for Vec<Vector<T>>
where
    T: Num + Copy,
{
    type Output = Self;
    fn transpose(&self) -> Self::Output {
        let mut b = vec![row![T::zero(); self.len()]; self[0].len()];
        for (i, vec) in self.iter().enumerate() {
            for (j, el) in vec.iter().enumerate() {
                b[j][i] = *el;
            }
        }
        b
    }
}
