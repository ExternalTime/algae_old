use super::{CorpusSet, LayoutEncoding, Metric};

pub struct Generator<K, const N: usize> {
    encoding: LayoutEncoding<K>,
    corpus_set: CorpusSet<N>,
    metric: Metric<N>,
}

impl<K, const N: usize> Generator<K, N>
where
    K: Clone + std::fmt::Debug + std::fmt::Display + Eq + 'static,
{
    pub fn new<I>(
        keyset: impl IntoIterator<Item = K>,
        charset: impl IntoIterator<Item = char>,
        corpora: impl IntoIterator<Item = I>,
        metric: impl Fn([K; N]) -> u64,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = ([char; N], u64)>,
    {
        let encoding = LayoutEncoding::new(keyset, charset)?;
        let corpora = corpora.into_iter().map(|corpus| {
            corpus.into_iter().flat_map(|(ngram, count)| {
                encoding
                    .chars
                    .encode_ngram(&ngram)
                    .map(|ngram| (ngram, count))
            })
        });
        let corpus_set = CorpusSet::new(corpora);
        let metric = Metric::new(encoding.len(), |nstroke| {
            metric(encoding.keys.decode_ngram(nstroke))
        });
        Ok(Self {
            encoding,
            corpus_set,
            metric,
        })
    }

    fn full_analysis(&self, layout: &[usize], buffer: &mut [u64]) -> f64 {
        debug_assert_eq!(buffer.len(), self.corpus_set.len());
        buffer.fill(0);
        for (ngram, counts) in self.corpus_set.iter() {
            let weight = self.metric.weight(ngram, layout);
            for (i, count) in counts.iter().enumerate() {
                buffer[i] += count * weight;
            }
        }
        self.corpus_set.aggregate_scores(buffer)
    }

    fn actual_generation(&self, mut layout: Vec<usize>, pinned: &[bool]) -> Vec<usize> {
        let mut buffer = vec![0; self.corpus_set.len()];
        let mut best_score = self.full_analysis(&layout, &mut buffer);
        let to_shuffle: Vec<_> = pinned
            .iter()
            .enumerate()
            .filter(|(_, &pinned)| !pinned)
            .map(|(i, _)| i)
            .collect();
        'outer: loop {
            let mut iter = to_shuffle.iter().copied();
            while let Some(i) = iter.next() {
                for j in iter.clone() {
                    layout.swap(i, j);
                    let score = self.full_analysis(&layout, &mut buffer);
                    if score < best_score {
                        best_score = score;
                        continue 'outer;
                    }
                    layout.swap(i, j);
                }
            }
            break;
        }
        layout
    }

    pub fn generate<L>(
        &self,
        pins: impl IntoIterator<Item = (K, char)>,
    ) -> Result<L, Box<dyn std::error::Error>>
    where
        L: FromIterator<(K, char)>,
    {
        let mut layout: Vec<_> = (0..self.encoding.len()).collect();
        let mut pinned = vec![false; self.encoding.len()];
        for (key, char) in pins {
            let (key, position) = (
                self.encoding.keys.encode(&key).unwrap(),
                self.encoding.chars.encode(&char).unwrap(),
            );
            let i = layout
                .iter()
                .enumerate()
                .find(|(_, x)| **x == key)
                .map(|(i, _)| i)
                .unwrap();
            assert!(!pinned[i] && !pinned[position]);
            layout.swap(i, position);
            pinned[position] = true;
        }
        //fastrand::shuffle(&mut layout);
        let layout = self.actual_generation(layout, &pinned);
        Ok(self.encoding.decode(layout).collect())
    }
}
