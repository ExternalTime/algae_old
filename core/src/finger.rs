#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Finger {
    pub kind: FingerKind,
    pub hand: Hand,
}

impl Finger {
    /// Returns mirrored finger
    ///
    /// # Example
    /// ```
    /// use algae_core::fingers;
    ///
    /// let left_pinky = fingers::LP;
    /// let right_pinky = left_pinky.mirror();
    /// assert_eq!(fingers::RP, right_pinky);
    /// ```
    pub fn mirror(&self) -> Self {
        Finger {
            kind: self.kind,
            hand: self.hand.mirror(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FingerKind {
    Pinky = 0,
    Ring = 1,
    Middle = 2,
    Index = 3,
    Thumb = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Hand {
    Left = 0,
    Right = 5,
}

impl Hand {
    pub fn mirror(&self) -> Self {
        use Hand::*;
        match self {
            Left => Right,
            Right => Left,
        }
    }
}

impl From<Finger> for u8 {
    fn from(f: Finger) -> Self {
        f.hand as u8 + f.kind as u8
    }
}

impl TryFrom<char> for Finger {
    type Error = String;
    fn try_from(v: char) -> Result<Self, Self::Error> {
        let v = if let Some(v) = v.to_digit(10) {
            v
        } else {
            return Err(format!(
                "{} is not a valid finger (digit from 0 to 9 inclusive)",
                v.escape_unicode(),
            ));
        };
        let v = usize::try_from(v).expect("number smaller than 10 should fit in usize");
        use FingerKind::*;
        use Hand::*;
        let hand = [Left, Right][v / 5];
        let kind = [Pinky, Ring, Middle, Index, Thumb][v % 5];
        Ok(Finger { hand, kind })
    }
}
