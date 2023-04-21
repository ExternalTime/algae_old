use super::{CorpusSet, InvalidLayoutEncoding, LayoutEncoding, Metric};

pub struct Generator<K, const N: usize> {
    encoding: LayoutEncoding<K>,
    corpus_set: CorpusSet<N>,
    metric: Metric<N>,
}

impl<K, const N: usize> Generator<K, N>
where
    K: Clone + Eq,
{
    pub fn new<I>(
        keyset: impl IntoIterator<Item = K>,
        charset: impl IntoIterator<Item = char>,
        corpora: impl IntoIterator<Item = I>,
        metric: impl Fn([K; N]) -> u64,
    ) -> Result<Self, InvalidLayoutEncoding<K>>
    where
        I: IntoIterator<Item = ([char; N], u64)>,
    {
        let encoding =
            LayoutEncoding::new(keyset.into_iter().collect(), charset.into_iter().collect())?;
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

    fn actual_generation<I>(&self, mut layout: Vec<usize>, not_pinned: I) -> Vec<usize>
    where
        I: Iterator<Item = usize> + Clone,
    {
        let mut buffer = vec![0; self.corpus_set.len()];
        let mut best_score = self.full_analysis(&layout, &mut buffer);
        'outer: loop {
            let mut iter = not_pinned.clone();
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

    pub fn generate<L>(&self, pins: impl IntoIterator<Item = (K, char)>) -> Result<L, InvalidPin<K>>
    where
        L: FromIterator<(K, char)>,
    {
        let len = self.encoding.len();
        let mut layout: Vec<_> = (0..len).collect();
        let mut pinned = vec![false; len];
        for (key, char) in pins {
            let (k, c) = (
                self.encoding
                    .keys
                    .encode(&key)
                    .ok_or(InvalidPin::InvalidKey(key.clone()))?,
                self.encoding
                    .chars
                    .encode(&char)
                    .ok_or(InvalidPin::InvalidChar(char))?,
            );
            let i = layout
                .iter()
                .enumerate()
                .find(|(_, x)| **x == k)
                .map(|(i, _)| i)
                .unwrap();
            if pinned[i] {
                return Err(InvalidPin::DuplicateKey(key));
            }
            if pinned[c] {
                return Err(InvalidPin::DuplicateChar(char));
            }
            layout.swap(i, c);
            pinned[c] = true;
        }
        let pins: Vec<_> = pinned
            .into_iter()
            .enumerate()
            .filter(|(_, pinned)| *pinned)
            .map(|(i, _)| i)
            .collect();
        let layout = match &*pins {
            [] => self.actual_generation(layout, 0..self.encoding.len()),
            pins => self.actual_generation(layout, pins.iter().copied()),
        };
        Ok(self.encoding.decode(layout).collect())
    }
}

#[derive(Debug)]
pub enum InvalidPin<K> {
    InvalidKey(K),
    DuplicateKey(K),
    InvalidChar(char),
    DuplicateChar(char),
}

use std::fmt::{self, Display, Formatter};
impl<T: Display> Display for InvalidPin<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use InvalidPin::*;
        match self {
            InvalidKey(key) => write!(f, "invalid key ({key})"),
            DuplicateKey(key) => write!(f, "duplicate key ({key})"),
            InvalidChar(char) => write!(f, "invalid char ({char})"),
            DuplicateChar(char) => write!(f, "duplicate char ({char})"),
        }
    }
}
