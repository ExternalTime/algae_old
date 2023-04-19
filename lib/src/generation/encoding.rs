#[derive(Clone, Debug)]
pub struct Encoding<T>(pub Box<[T]>);

impl<T> Encoding<T>
where
    T: Clone + Eq,
{
    pub fn new(value_set: impl IntoIterator<Item = T>) -> Result<Self, Vec<T>> {
        let mut values = Vec::new();
        let mut duplicates = Vec::new();
        for v in value_set {
            if !values.contains(&v) {
                values.push(v);
            } else if !duplicates.contains(&v) {
                duplicates.push(v);
            }
        }
        if !duplicates.is_empty() {
            Err(duplicates)
        } else {
            Ok(Self(values.into_boxed_slice()))
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn encode(&self, v: &T) -> Option<usize> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, w)| *w == v)
            .map(|(i, _)| i)
    }

    pub fn decode(&self, e: usize) -> &T {
        &self.0[e]
    }

    pub fn encode_ngram<const N: usize>(&self, ngram: &[T; N]) -> Option<[usize; N]> {
        let mut res = [0; N];
        for (i, val) in res.iter_mut().zip(ngram.iter()) {
            *i = self.encode(val)?;
        }
        Some(res)
    }

    pub fn decode_ngram<const N: usize>(&self, ngram: [usize; N]) -> [T; N] {
        ngram.map(|val| self.decode(val).clone())
    }
}
