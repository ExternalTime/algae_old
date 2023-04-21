#[derive(Clone, Debug)]
pub struct Encoding<T>(pub Box<[T]>);

impl<T> Encoding<T>
where
    T: Clone + Eq,
{
    pub fn new(mut values: Vec<T>) -> Result<Self, Vec<T>> {
        let mut duplicates = 0;
        for i in 1..values.len() {
            if values[duplicates..i].contains(&values[i]) {
                values.swap(duplicates, i);
                duplicates += 1;
            }
        }
        match duplicates {
            0 => Ok(Self(values.into_boxed_slice())),
            _ => {
                values.truncate(duplicates);
                Err(values)
            }
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
