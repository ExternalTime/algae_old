use super::Encoding;
use std::cmp::Eq;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub struct LayoutEncoding<K> {
    pub keys: Encoding<K>,
    pub chars: Encoding<char>,
}

impl<K: Clone + Eq> LayoutEncoding<K> {
    pub fn new(keys: Vec<K>, chars: Vec<char>) -> Result<Self, InvalidLayoutEncoding<K>> {
        let counts = match keys.len() == chars.len() {
            true => None,
            false => Some([keys.len(), chars.len()]),
        };
        match (Encoding::new(keys), Encoding::new(chars), counts) {
            (Ok(keys), Ok(chars), None) => Ok(Self { keys, chars }),
            (keys, chars, different_counts) => Err(InvalidLayoutEncoding {
                duplicate_keys: keys.err().unwrap_or(Vec::new()),
                duplicate_chars: chars.err().unwrap_or(Vec::new()),
                different_counts,
            }),
        }
    }

    pub fn decode(&self, vec: Vec<usize>) -> impl Iterator<Item = (K, char)> + '_ {
        vec.into_iter()
            .enumerate()
            .map(|(c, k)| (self.keys.decode(k).clone(), *self.chars.decode(c)))
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }
}

#[derive(Clone, Debug)]
pub struct InvalidLayoutEncoding<K> {
    duplicate_keys: Vec<K>,
    duplicate_chars: Vec<char>,
    different_counts: Option<[usize; 2]>,
}

impl<K: Display> Display for InvalidLayoutEncoding<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fn print_duplicates<T: Display>(
            f: &mut Formatter<'_>,
            vec: &[T],
            label: &str,
        ) -> fmt::Result {
            let mut iter = vec.iter();
            if let Some(head) = iter.next() {
                write!(f, "Duplicate {label}s: {head}")?;
                for value in iter {
                    write!(f, ", {value}")?;
                }
                writeln!(f, ".")?;
            }
            Ok(())
        }
        writeln!(f, "Invalid layout encoding.")?;
        if let Some([l1, l2]) = self.different_counts {
            writeln!(f, "Number of keys ({l1}) and chars ({l2}) are different!")?;
        }
        print_duplicates(f, &self.duplicate_keys, "key")?;
        print_duplicates(f, &self.duplicate_chars, "char")?;
        Ok(())
    }
}

impl<K: Display + Debug> std::error::Error for InvalidLayoutEncoding<K> {}
