use super::{CompiledNgramAnalyzer, Encoding};
use std::cmp::Eq;
use std::ops::{AddAssign, Mul};
use std::fmt::Debug;

#[derive(Debug)]
pub struct MapEncoding<K, V> {
    keys: Encoding<K>,
    values: Encoding<V>,
    pins: usize,
}

impl<K, V> MapEncoding<K, V>
where
    K: Clone + Eq + Debug,
    V: Clone + Eq + Debug,
{
    pub fn new(
        key_set: impl IntoIterator<Item = K>,
        value_set: impl IntoIterator<Item = V>,
        pinned: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let (keys, values): (Vec<K>, Vec<V>) = pinned.into_iter().unzip();
        let pins = keys.len();
        let keys = Encoding::new(keys, key_set);
        let values = Encoding::new(values, value_set);
        assert_eq!(keys.len(), values.len(), "map encoding must contain same number of keys and characters - {} != {}", keys.len(), values.len());
        Self { keys, values, pins }
    }

    pub fn encode(&self, map: impl Fn(&K) -> &V) -> Vec<usize> {
        let mut vec = Vec::new();
        for key in self.keys.0.iter() {
            vec.push(self.values.encode(map(key)));
        }
        vec
    }

    pub fn decode(&self, vec: Vec<usize>) -> impl Iterator<Item = (K, V)> + '_ {
        vec.into_iter()
            .enumerate()
            .map(|(k, v)| (self.keys.decode(k).clone(), self.values.decode(v).clone()))
    }

    pub fn pins(&self) -> usize {
        self.pins
    }

    pub fn weight_calculator<Score, const D: usize>(
        &self,
        wk: impl Fn([K; D]) -> Score,
        wv: impl Fn([V; D]) -> Score,
    ) -> CompiledNgramAnalyzer<Score, D>
    where
        Score: Default + Mul<Output = Score> + AddAssign + Copy + Eq,
    {
        let sparse = self.keys.encode_into_sparse_tensor(wk);
        let dense = self.values.encode_into_tensor(wv);
        CompiledNgramAnalyzer::new(dense, sparse, self.pins, self.keys.len())
    }
}
