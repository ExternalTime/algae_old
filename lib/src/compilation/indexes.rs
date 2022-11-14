/// Iterator over indexes into a square tensor.
pub struct Indexes<const DIM: usize> {
    len: usize,
    val: [usize; DIM],
}

impl<const DIM: usize> Indexes<DIM> {
    pub fn new(len: usize) -> Self {
        Self { len, val: [0; DIM] }
    }
}

impl<const DIM: usize> Iterator for Indexes<DIM> {
    type Item = [usize; DIM];
    fn next(&mut self) -> Option<Self::Item> {
        if self.val[DIM - 1] < self.len {
            let res = self.val;
            for i in self.val.iter_mut().rev() {
                if *i < self.len - 1 {
                    *i += 1;
                    return Some(res);
                } else {
                    *i = 0;
                }
            }
            self.val[DIM - 1] = self.len;
            Some(res)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len_0() {
        let mut indexes = Indexes::<3>::new(0);
        assert_eq!(indexes.next(), None);
    }

    #[test]
    fn len_1() {
        let mut indexes = Indexes::new(1);
        assert_eq!(indexes.next(), Some([0, 0, 0]));
        assert_eq!(indexes.next(), None);
    }

    #[test]
    fn indexes_order() {
        let mut indexes = Indexes::new(2);
        assert_eq!(indexes.next(), Some([0, 0, 0]));
        assert_eq!(indexes.next(), Some([0, 0, 1]));
        assert_eq!(indexes.next(), Some([0, 1, 0]));
        assert_eq!(indexes.next(), Some([0, 1, 1]));
        assert_eq!(indexes.next(), Some([1, 0, 0]));
        assert_eq!(indexes.next(), Some([1, 0, 1]));
        assert_eq!(indexes.next(), Some([1, 1, 0]));
        assert_eq!(indexes.next(), Some([1, 1, 1]));
        assert_eq!(indexes.next(), None);
    }

    #[test]
    fn higher_indexes() {
        let mut indexes = Indexes::new(3);
        assert_eq!(indexes.next(), Some([0, 0]));
        assert_eq!(indexes.next(), Some([0, 1]));
        assert_eq!(indexes.next(), Some([0, 2]));
        assert_eq!(indexes.next(), Some([1, 0]));
        assert_eq!(indexes.next(), Some([1, 1]));
        assert_eq!(indexes.next(), Some([1, 2]));
        assert_eq!(indexes.next(), Some([2, 0]));
        assert_eq!(indexes.next(), Some([2, 1]));
        assert_eq!(indexes.next(), Some([2, 2]));
        assert_eq!(indexes.next(), None);
    }
}
