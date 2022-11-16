use super::{CompiledNgramAnalyzer, Encoding};
use std::cmp::Eq;
use std::fmt::Debug;
use std::ops::{AddAssign, Mul};

/// At runtimes defines layout encoding to allow more efficient analysis.

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
    /// Creates new encoding.
    ///
    /// # Panics
    ///
    /// This function panics if any of the arguments contains duplicates or
    /// key and value sets don't match in length.
    pub fn new(
        key_set: impl IntoIterator<Item = K>,
        value_set: impl IntoIterator<Item = V>,
        pinned: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let (keys, values): (Vec<K>, Vec<V>) = pinned.into_iter().unzip();
        let pins = keys.len();
        let keys = Encoding::new(keys, key_set);
        let values = Encoding::new(values, value_set);
        assert_eq!(
            keys.len(),
            values.len(),
            "map encoding must contain same number of keys and characters - {} != {}",
            keys.len(),
            values.len()
        );
        Self { keys, values, pins }
    }

    /// Encodes a layout.
    pub fn encode(&self, map: impl Fn(&K) -> &V) -> Vec<usize> {
        let mut vec = Vec::new();
        for key in self.keys.0.iter() {
            vec.push(self.values.encode(map(key)));
        }
        vec
    }

    /// Decodes a layout.
    pub fn decode(&self, vec: Vec<usize>) -> impl Iterator<Item = (K, V)> + '_ {
        vec.into_iter()
            .enumerate()
            .map(|(k, v)| (self.keys.decode(k).clone(), self.values.decode(v).clone()))
    }

    /// Returns the number of pins.
    pub fn pins(&self) -> usize {
        self.pins
    }

    pub fn compile_analyzer<Score, WK, WV, const D: usize>(
        &self,
        wk: impl Fn([K; D]) -> WK,
        wv: impl Fn([V; D]) -> WV,
    ) -> CompiledNgramAnalyzer<WK, WV, D>
    where
        WK: Clone + Default,
        WV: Mul<WK, Output = Score> + Clone + Default + Eq,
        Score: Default + AddAssign + Clone + Ord,
    {
        let dense = self.keys.encode_into_tensor(wk);
        let sparse = self.values.encode_into_sparse_tensor(wv);
        CompiledNgramAnalyzer::new(dense, sparse, self.pins, self.keys.len())
    }
}
