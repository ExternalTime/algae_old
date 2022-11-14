use std::ops::{AddAssign, Index, Mul};

pub struct CompiledNgramAnalyzer<Score, const N: usize> {
    // Sparse tensor
    sparse: Vec<([usize; N], Score)>,
    // Bounds for incremental analysis
    bucket_bounds: Vec<usize>,

    // Tensor represented as a flat array
    dense: Box<[Score]>,
    side_len: usize,
}

impl<Score, const N: usize> CompiledNgramAnalyzer<Score, N>
where
    Score: Mul<Output = Score> + AddAssign + Default + Copy,
{
    pub fn new(
        dense: Box<[Score]>,
        sparse: Vec<([usize; N], Score)>,
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
        Vec<([usize; N], Score)>: Index<R, Output = [([usize; N], Score)]>,
    {
        let mut score = Score::default();
        for (index, weight) in self.sparse[range].iter() {
            score += self.dense_weight(index, layout) * *weight;
        }
        score
    }

    fn dense_weight(&self, index: &[usize; N], layout: &[usize]) -> Score {
        let mut res = 0;
        for &i in index {
            res *= self.side_len;
            res += layout[i];
        }
        self.dense[res]
    }
}
