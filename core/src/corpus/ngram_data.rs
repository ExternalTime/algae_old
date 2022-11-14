use super::Windows;
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
    // Remembers which ngram is first (it's contained in both maps).
    // Helps accurately calculate shorter ngrams. Especially
    // important when dealing with many short texts.
    heads: Ngrams<N>,
    ngrams: Ngrams<N>,
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

    /// Creates empty `NgramData`.
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

    /// Adds occurences from supplied text onto self.
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
        let iter = iter.into_iter();
        let mut ngrams = Windows::new(iter);
        let head = match ngrams.next() {
            Some(head) => head,
            None => todo!("Should decide what should happen here. Panicking works for now"),
        };
        *self.ngrams.entry(head).or_insert(0) += 1;
        *self.heads.entry(head).or_insert(0) += 1;
        for ngram in ngrams {
            *self.ngrams.entry(ngram).or_insert(0) += 1;
        }
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
    pub fn expand(&self, expansion: impl Fn(char) -> Vec<char>) -> Self {
        let heads = self.expand_heads(&expansion);
        let ngrams = self.expand_ngrams(&expansion);
        Self { heads, ngrams }
    }

    fn expand_heads(&self, expansion: &impl Fn(char) -> Vec<char>) -> Ngrams<N> {
        let mut heads = Ngrams::new();
        for (&head, &occurences) in self.heads.iter() {
            let expanded = head.into_iter().flat_map(expansion);
            let mut windows = Windows::new(expanded);
            let head = windows
                .next()
                .expect("expansion of each character should be at least 1-character long");
            *heads.entry(head).or_insert(0) += occurences;
        }
        heads
    }

    fn expand_ngrams(&self, expansion: &impl Fn(char) -> Vec<char>) -> Ngrams<N> {
        let mut ngrams = Ngrams::new();
        for (&ngram, &occurences) in self.ngrams.iter() {
            let to_add = expansion(ngram[0]).len();
            let expanded = ngram.into_iter().flat_map(expansion);
            let mut windows = Windows::new(expanded);
            for _ in 0..to_add {
                let ngram = windows
                    .next()
                    .expect("expansion of each character should be at least 1-character long");
                *ngrams.entry(ngram).or_insert(0) += occurences;
            }
        }
        ngrams
    }

    /// Cheaply convert self into engrams.
    ///
    /// # Example
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let trigrams = NgramData::<3>::new("Quick Fox".chars());
    /// let trigrams = trigrams.into_inner();
    /// assert_eq!(trigrams.get(&['Q', 'u', 'i']), Some(&1));
    /// assert_eq!(trigrams.get(&['a', 'b', 'c']), None);
    /// ```
    pub fn into_inner(self) -> Ngrams<N> {
        self.ngrams
    }

    /// Calculates ngrams of length K
    ///
    /// # Panics
    /// This function will panic if N < K.
    ///
    /// # Examples
    ///
    /// ```
    /// use algae_core::NgramData;
    /// let trigrams = NgramData::<3>::new("Quick Fox".chars());
    /// let bigrams = trigrams.ngrams::<2>();
    /// assert_eq!(bigrams.get(&['Q', 'u']), Some(&1));
    /// ```
    pub fn ngrams<const K: usize>(&self) -> Ngrams<K> {
        assert!(K <= N);
        let mut ngrams = Ngrams::new();
        // To get an accurate count we only get last ngram from `ngrams`
        // and all except last from `heads`.
        for (head, occurences) in self.heads.iter() {
            for i in 0..(N - K) {
                *ngrams
                    .entry(head[i..][..K].try_into().unwrap())
                    .or_insert(0) += occurences;
            }
        }
        for (ngram, occurences) in self.ngrams.iter() {
            *ngrams
                .entry(ngram[(N - K)..].try_into().unwrap())
                .or_insert(0) += occurences;
        }
        ngrams
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
        let text = "Aaaaa";
        let expected: Ngrams<2> = [(['A', 'a'], 1), (['a', 'a'], 3)].into_iter().collect();
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
}
