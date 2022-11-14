use crate::{Finger, FingerKind::*, Hand::*};

/// Left pinky
pub const LP: Finger = Finger {
    kind: Pinky,
    hand: Left,
};
/// Left ring
pub const LR: Finger = Finger {
    kind: Ring,
    hand: Left,
};
/// Left middle
pub const LM: Finger = Finger {
    kind: Middle,
    hand: Left,
};
/// Left index
pub const LI: Finger = Finger {
    kind: Index,
    hand: Left,
};
/// Left thumb
pub const LT: Finger = Finger {
    kind: Thumb,
    hand: Left,
};
/// Right pinky
pub const RP: Finger = Finger {
    kind: Pinky,
    hand: Right,
};
/// Right ring
pub const RR: Finger = Finger {
    kind: Ring,
    hand: Right,
};
/// Right middle
pub const RM: Finger = Finger {
    kind: Middle,
    hand: Right,
};
/// Right index
pub const RI: Finger = Finger {
    kind: Index,
    hand: Right,
};
/// Right thumb
pub const RT: Finger = Finger {
    kind: Thumb,
    hand: Right,
};
