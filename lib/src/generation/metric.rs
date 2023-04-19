pub struct Metric<const N: usize> {
    data: Box<[u64]>,
    side: usize,
}

impl<const N: usize> Metric<N> {
    pub fn new(side: usize, weight: impl Fn([usize; N]) -> u64) -> Self {
        let total_size = side.checked_pow(N.try_into().unwrap()).unwrap();
        let mut data = Vec::with_capacity(total_size);
        let mut key = [0; N];
        'outer: loop {
            data.push(weight(key));
            for i in key.iter_mut().rev() {
                if *i < side - 1 {
                    *i += 1;
                    continue 'outer;
                }
                *i = 0;
            }
            break;
        }
        assert_eq!(total_size, data.len());
        Self {
            data: data.into_boxed_slice(),
            side,
        }
    }

    pub fn weight(&self, ngram: &[usize; N], layout: &[usize]) -> u64 {
        self.data[ngram.iter().fold(0, |acc, i| acc * self.side + layout[*i])]
    }
}
