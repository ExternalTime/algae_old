use super::Indexes;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Encoding<T>(pub Box<[T]>);

impl<T> Encoding<T>
where
    T: Clone + Eq + Debug,
{
    pub fn new(mut pins: Vec<T>, value_set: impl IntoIterator<Item = T>) -> Self {
        // Checking for errors
        let value_set: Vec<_> = value_set.into_iter().collect();
        let duplicates = {
            let mut iter = value_set.iter();
            let mut duplicates = Vec::new();
            while let Some(v) = iter.next() {
                if iter.clone().any(|x| x == v) {
                    duplicates.push(v.clone());
                }
            }
            duplicates
        };
        assert!(duplicates.is_empty(), "encountered duplicates: {:?}", duplicates);
        let invalid: Vec<T> = pins.iter().filter(|x| !value_set.contains(x)).cloned().collect();
        assert!(invalid.is_empty(), "encountered invalid values: {:?}", invalid);
        
        // This way we preserve original ordering while moving pins to the start
        for v in value_set {
            if !pins.contains(&v) {
                pins.push(v);
            }
        }
        Self(pins.into_boxed_slice())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn encode(&self, v: &T) -> usize {
        self.0.iter().enumerate().find(|(_, x)| x == &v).unwrap().0
    }

    pub fn decode(&self, e: usize) -> &T {
        &self.0[e]
    }

    pub fn encode_into_tensor<Num, const D: usize>(
        &self,
        weight: impl Fn([T; D]) -> Num,
    ) -> Box<[Num]>
    where
        Num: Default + Clone,
    {
        let mut array =
            vec![Default::default(); self.len().saturating_pow(D as u32)].into_boxed_slice();
        for (flat, i) in Indexes::<D>::new(self.len()).enumerate() {
            let decoded_index: [T; D] = i.map(|v| self.decode(v).clone());
            array[flat] = weight(decoded_index);
        }
        array
    }

    pub fn encode_into_sparse_tensor<Num, const D: usize>(
        &self,
        weight: impl Fn([T; D]) -> Num,
    ) -> Vec<([usize; D], Num)>
    where
        Num: Default + Eq,
    {
        let mut tensor = Vec::new();
        for i in Indexes::<D>::new(self.len()) {
            let decoded_index = i.map(|v| self.decode(v).clone());
            let weight = weight(decoded_index);
            if weight != Num::default() {
                tensor.push((i, weight));
            }
        }
        tensor
    }
}
