#[derive(Clone, Debug)]
pub struct CorpusSet<const N: usize> {
    ngrams: Vec<[usize; N]>,
    weights: Vec<u64>,
    weight_sums: Vec<f64>,
    corpus_count: usize,
}

impl<const N: usize> CorpusSet<N> {
    pub fn new<I>(corpora: impl Iterator<Item = I>) -> Self
    where
        I: IntoIterator<Item = ([usize; N], u64)>,
    {
        let corpora: Vec<_> = corpora.into_iter().collect();
        let len = corpora.len();
        let mut ngrams = Vec::new();
        let mut weights = Vec::new();
        let mut weight_sums = vec![0; len];
        for (i, ngram, count) in corpora.into_iter().enumerate().flat_map(|(i, corpus)| {
            corpus
                .into_iter()
                .map(move |(ngram, count)| (i, ngram, count))
        }) {
            let j = match ngrams
                .iter_mut()
                .enumerate()
                .find(|(_, x)| **x == ngram)
                .map(|(i, _)| i)
            {
                Some(j) => j,
                None => {
                    ngrams.push(ngram);
                    weights.resize(weights.len() + len, 0);
                    ngrams.len() - 1
                }
            };
            weights[j * len + i] = count;
            weight_sums[i] += count;
        }
        assert_eq!(weights.len(), ngrams.len() * len);
        let weight_sums = weight_sums
            .into_iter()
            .map(|x| if x < 1 { 1 } else { x } as f64)
            .collect();
        Self {
            ngrams,
            weights,
            weight_sums,
            corpus_count: len,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&[usize; N], &[u64])> {
        self.ngrams
            .iter()
            .zip(self.weights.chunks_exact(self.corpus_count))
    }

    pub fn aggregate_scores(&self, scores: &[u64]) -> f64 {
        scores
            .iter()
            .zip(self.weight_sums.iter())
            .map(|(score, sum)| *score as f64 / sum)
            .max_by(|&f1, f2| f1.partial_cmp(f2).unwrap())
            .unwrap()
    }

    pub fn len(&self) -> usize {
        self.corpus_count
    }
}
