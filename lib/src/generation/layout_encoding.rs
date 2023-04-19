use super::Encoding;
use std::cmp::Eq;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub struct LayoutEncoding<K> {
    pub keys: Encoding<K>,
    pub chars: Encoding<char>,
}

impl<K: Clone + Eq + Debug + Display + 'static> LayoutEncoding<K> {
    pub fn new(
        keys: impl IntoIterator<Item = K>,
        chars: impl IntoIterator<Item = char>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let (keys, chars) = match (Encoding::new(keys), Encoding::new(chars)) {
            (Ok(keys), Ok(chars)) => (keys, chars),
            (keys, chars) => {
                return Err(Box::new(InvalidLayoutEncoding::Duplicates((
                    keys.err().unwrap_or(Vec::new()),
                    chars.err().unwrap_or(Vec::new()),
                ))))
            }
        };
        if keys.len() != chars.len() {
            Err(Box::new(InvalidLayoutEncoding::<K>::DifferentCounts([
                keys.len(),
                chars.len(),
            ])))
        } else {
            Ok(Self { keys, chars })
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

#[derive(Clone, Debug, PartialEq, Eq)]
enum InvalidLayoutEncoding<K> {
    Duplicates((Vec<K>, Vec<char>)),
    DifferentCounts([usize; 2]),
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
        use InvalidLayoutEncoding::*;
        writeln!(f, "Invalid layout encoding.")?;
        match self {
            Duplicates((v1, v2)) => {
                print_duplicates(f, v1, "key")?;
                print_duplicates(f, v2, "char")?;
            }
            DifferentCounts([l1, l2]) => writeln!(
                f,
                "The numbers of keys ({l1}) and characters ({l2}) are different."
            )?,
        }
        Ok(())
    }
}

impl<K: Display + Debug> std::error::Error for InvalidLayoutEncoding<K> {}
