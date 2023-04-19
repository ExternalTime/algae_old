use super::{expand_first, expand_full};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type Ngrams<const N: usize> = HashMap<[char; N], u64>;

/// Calculates ngrams of a corpora. Can be used to get accurate number of ngrams up to `N` long.
///
/// # Examples
///
/// Typical usage:
/// ```
/// use algae_core::NgramData;
///
/// // Load the corpus
/// let corpus = "Quick brown fox jumps over a lazy dog";
/// let trigrams = NgramData::<3>::new(corpus.chars());
///
/// // Expand characters where needed. Here for example we
/// // are expanding capital letters into a shift placeholder
/// // and lowercase letter. Keep in mind that chosen
/// // placeholder could occur naturally in the corpus.
/// let trigrams = trigrams.expand(|char|
///     if char.is_ascii_uppercase() {
///         vec!['⇧', char.to_ascii_lowercase()]
///     } else {
///         vec![char]
/// });
///
/// // Get raw ngrams
/// let bigrams = trigrams.ngrams::<2>();
/// let trigrams = trigrams.into_inner();
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(serialize = "[char; N]: Serialize"))]
#[serde(bound(deserialize = "[char; N]: Deserialize<'de>"))]
pub struct NgramData<const N: usize> {
    // Last ngrams (tails) are stored separately from the rest to allow
    // us to accurately calculate expansion results and shorter ngrams.
    ngrams: Ngrams<N>,
    tails: Ngrams<N>,
}

impl<const N: usize> NgramData<N> {
    /// Creates empty `NgramData`.
    ///
    /// # Example
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let corpus = NgramData::<3>::empty();
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }

    /// Calculates `NgramData` from supplied iterator.
    ///
    /// # Panics
    ///
    /// This function will panic if supplied iterator is shorter than `N`.
    ///
    /// # Example
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let corpus = NgramData::<3>::new("Quick Fox".chars());
    /// ```
    pub fn new(iter: impl IntoIterator<Item = char>) -> Self {
        Self::from_iter(iter)
    }

    /// Counts ngrams in supplied text.
    ///
    /// # Panics
    ///
    /// This function will panic if supplied iterator is shorter than `N`.
    ///
    /// # Example
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let mut corpus = NgramData::<3>::empty();
    /// corpus.add("Quick Fox".chars());
    /// ```
    pub fn add(&mut self, iter: impl IntoIterator<Item = char>) {
        let mut iter = iter.into_iter();
        let mut vec = Vec::with_capacity(N);
        for _ in 0..N {
            match iter.next() {
                Some(char) => vec.push(char),
                // TODO: Decide on how this situation should be handled.
                None => unimplemented!("tried to count ngrams in text shorter than N"),
            }
        }
        let mut ngram: [char; N] = vec.try_into().unwrap();
        for char in iter {
            *self.ngrams.entry(ngram).or_insert(0) += 1;
            ngram.rotate_left(1);
            ngram[N - 1] = char;
        }
        *self.tails.entry(ngram).or_insert(0) += 1;
    }

    /// Creates `NgramData` from self where characters get expanded by `expansion`.
    ///
    /// # Panics
    ///
    /// The function *may* panic if `expansion` returns empty vec for any character.
    ///
    /// # Example
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let text = "Quick Fox";
    /// let trigrams = NgramData::<3>::new(text.chars());
    /// let trigrams = trigrams.expand(|char|
    ///     if char.is_ascii_uppercase() {
    ///         vec!['⇧', char.to_ascii_lowercase()]
    ///     } else {
    ///         vec![char]
    ///     }
    /// );
    /// let trigrams = trigrams.into_inner();
    /// assert_eq!(trigrams.get(&['Q', 'u', 'i']), None);
    /// assert_eq!(trigrams.get(&['⇧', 'q', 'u']), Some(&1));
    /// assert_eq!(trigrams.get(&[' ', '⇧', 'f']), Some(&1));
    /// ```
    pub fn expand<F, Iter>(&self, expand: F) -> Self
    where
        F: Fn(char) -> Iter,
        Iter: IntoIterator<Item = char>,
    {
        let expand = &|char: &char| expand(*char);
        let mut tails = Ngrams::new();
        let mut ngrams = Ngrams::new();
        for (tail, &count) in &self.tails {
            let mut expanded = expand_full(tail, expand).into_iter().rev();
            *tails.entry(expanded.next().unwrap()).or_insert(0) += count;
            for ngram in expanded {
                *ngrams.entry(ngram).or_insert(0) += count;
            }
        }
        for (ngram, &count) in &self.ngrams {
            for ngram in expand_first(ngram, expand) {
                *ngrams.entry(ngram).or_insert(0) += count;
            }
        }
        Self { ngrams, tails }
    }

    /// Calculates the number of ngrams of length `K`
    ///
    /// # Panics
    /// This function will panic if N < K.
    ///
    /// # Examples
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let corpus = "Quick brown fox";
    /// let trigrams = NgramData::<3>::new(corpus.chars());
    /// let bigrams = NgramData::<2>::new(corpus.chars()).into_inner();
    /// let contracted = trigrams.ngrams::<2>();
    /// assert_eq!(bigrams, contracted);
    /// ```
    pub fn ngrams<const K: usize>(&self) -> Ngrams<K> {
        assert!(K <= N);
        self.tails
            .iter()
            .flat_map(|(tail, count)| {
                tail.windows(K)
                    .map(move |window| (window.try_into().unwrap(), count))
            })
            .chain(
                self.ngrams
                    .iter()
                    .map(|(ngram, count)| (ngram[..K].try_into().unwrap(), count)),
            )
            .fold(Ngrams::new(), |mut ngrams, (ngram, &count)| {
                *ngrams.entry(ngram).or_insert(0) += count;
                ngrams
            })
    }
}

impl<const N: usize> FromIterator<char> for NgramData<N> {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut res = Self::empty();
        res.add(iter.into_iter());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn trigrams() {
        let text = "Aaaaa";
        let expected: Ngrams<3> = [(['A', 'a', 'a'], 1), (['a', 'a', 'a'], 2)]
            .into_iter()
            .collect();
        let trigrams = NgramData::new(text.chars());
        let trigrams = trigrams.into_inner();
        assert_eq!(trigrams, expected);
    }

    #[test]
    pub fn shorter_ngrams() {
        let text = "Quick fox";
        let expected: Ngrams<2> = NgramData::new(text.chars()).into_inner();
        let trigrams = NgramData::<3>::new(text.chars());
        let bigrams = trigrams.ngrams();
        assert_eq!(bigrams, expected);
    }

    #[test]
    pub fn no_expansion() {
        let text = "quick fox";
        let before = NgramData::<3>::new(text.chars());
        let after = before.expand(|char| vec![char]);
        assert_eq!(after, before);
    }

    #[test]
    pub fn simple_expansion() {
        let text = "Quick fox";
        let expansion = |char| vec!['.', char];
        let ngrams = NgramData::<2>::new(text.chars())
            .expand(expansion)
            .into_inner();
        let expected = NgramData::<3>::new(text.chars().flat_map(expansion)).ngrams::<2>();
        assert_eq!(ngrams, expected);
    }
}
