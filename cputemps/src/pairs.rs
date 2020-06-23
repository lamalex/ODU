pub trait Pairs<T: Clone> {
    fn pairs(self) -> PairIterator<T, Self>
    where
        Self: Sized;
}

pub struct PairIterator<T: Clone, I> {
    iter: I,
    previous: Option<T>,
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for PairIterator<T, I> {
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.previous.as_ref()?;

        match self.iter.next() {
            Some(next) => {
                let previous = self.previous.replace(next.clone()).unwrap();
                Some((previous, next))
            }
            None => None,
        }
    }
}

impl<T, I> PairIterator<T, I>
where
    T: Clone,
    I: Iterator<Item = T>,
{
    pub fn new(mut iter: I) -> PairIterator<T, I> {
        let previous = iter.next();
        PairIterator { iter, previous }
    }
}

impl<T, I> Pairs<T> for I
where
    I: Iterator<Item = T> + Sized,
    T: Clone,
{
    fn pairs(self) -> PairIterator<T, Self>
    where
        Self: Sized,
    {
        PairIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pair_iter() {
        let mut sut = [1, 2, 3, 4].iter().pairs();
        assert_eq!((&1, &2), sut.next().unwrap());
    }

    #[test]
    fn test_pair_iter_none() {
        let mut sut: PairIterator<&i32, _> = [].iter().pairs();
        assert_eq!(None, sut.next())
    }
}
