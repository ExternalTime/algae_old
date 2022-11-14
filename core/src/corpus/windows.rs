pub struct Windows<T, I, const N: usize> {
    inner: I,
    array: [T; N],
}

impl<T, I, const N: usize> Windows<T, I, N>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    pub fn new<IntoIter>(iter: IntoIter) -> Self
    where
        IntoIter: IntoIterator<IntoIter = I>,
    {
        let mut inner = iter.into_iter();
        let mut array = [T::default(); N];
        for i in array.iter_mut().take(N).skip(1) {
            if let Some(e) = inner.next() {
                *i = e;
            }
        }
        Self { inner, array }
    }
}

impl<T, I, const N: usize> Iterator for Windows<T, I, N>
where
    I: Iterator<Item = T>,
    T: Copy + Default,
{
    type Item = [T; N];
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.inner.next() {
            self.array.rotate_left(1);
            self.array[N - 1] = item;
            Some(self.array)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_for_shorter() {
        let mut windows = Windows::<_, _, 4>::new([1, 2, 3]);
        assert!(windows.next().is_none());
    }

    #[test]
    fn single_window() {
        let mut windows = Windows::<_, _, 2>::new([1, 2]);
        assert_eq!(windows.next(), Some([1, 2]));
        assert!(windows.next().is_none())
    }

    #[test]
    fn multiple_windows() {
        let sequence = [1, 2, 3, 4, 5];
        let mut windows = Windows::new(sequence.into_iter());
        assert_eq!(windows.next(), Some([1, 2]));
        assert_eq!(windows.next(), Some([2, 3]));
        assert_eq!(windows.next(), Some([3, 4]));
        assert_eq!(windows.next(), Some([4, 5]));
        assert_eq!(windows.next(), None);
    }
}
