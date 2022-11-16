use std::ops::{AddAssign, Index, Mul};

pub struct CompiledNgramAnalyzer<WK, WV, const N: usize> {
    // Sparse tensor
    sparse: Vec<([usize; N], WV)>,
    // Bounds for incremental analysis
    bucket_bounds: Vec<usize>,

    // Tensor represented as a flat array
    dense: Box<[WK]>,
    side_len: usize,
}

impl<Score, WK, WV, const N: usize> CompiledNgramAnalyzer<WK, WV, N>
where
    WV: Mul<WK, Output = Score> + Clone,
    WK: Clone,
    Score: AddAssign + Default + Clone,
{
    pub fn new(
        dense: Box<[WK]>,
        sparse: Vec<([usize; N], WV)>,
        pins: usize,
        side_len: usize,
    ) -> Self {
        assert!(pins < side_len);
        let mut buckets: Vec<_> = std::iter::repeat_with(Vec::new).take(side_len).collect();
        for pair in sparse {
            buckets[*pair.0.iter().max().unwrap()].push(pair);
        }

        let mut bucket_bounds = vec![0; side_len + 1];
        let mut sparse = Vec::new();
        for (i, mut bucket) in buckets.into_iter().enumerate().skip(pins) {
            sparse.append(&mut bucket);
            bucket_bounds[i + 1] = sparse.len();
        }

        Self {
            sparse,
            bucket_bounds,
            dense,
            side_len,
        }
    }

    pub fn step_score(&self, layout: &[usize]) -> Score {
        let bounds = &self.bucket_bounds[layout.len() - 1..];
        let bounds = bounds[0]..bounds[1];
        self.score_inner(layout, bounds)
    }

    pub fn score(&self, layout: &[usize]) -> Score {
        self.score_inner(layout, ..)
    }

    fn score_inner<R>(&self, layout: &[usize], range: R) -> Score
    where
        Vec<([usize; N], WV)>: Index<R, Output = [([usize; N], WV)]>,
    {
        let mut score = Score::default();
        for (index, weight) in self.sparse[range].iter() {
            score += weight.clone() * self.dense_weight(index, layout);
        }
        score
    }

    fn dense_weight(&self, index: &[usize; N], layout: &[usize]) -> WK {
        let mut res = 0;
        for &i in index {
            res *= self.side_len;
            res += layout[i];
        }
        self.dense[res].clone()
    }
}
