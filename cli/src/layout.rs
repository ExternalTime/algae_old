use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key(pub [usize; 2]);

#[derive(Clone, Debug)]
pub struct Layout([[char; 10]; 3]);

impl FromIterator<(Key, char)> for Layout {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Key, char)>,
    {
        let mut res = [['\0'; 10]; 3];
        for (Key([row, col]), char) in iter {
            res[row][col] = char;
        }
        Self(res)
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Key([row, col]) = self;
        write!(f, "({col}, {row})")
    }
}

impl Display for Layout {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            let mut iter = row.iter();
            write!(f, "{}", iter.next().unwrap())?;
            for key in iter {
                write!(f, " {key}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
