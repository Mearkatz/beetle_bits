//! Crate for performing operations on bits and bytes

use std::fmt;

/**
Behaves like a bool, but appears as the integer 0 or 1 when printed

# Examples
```
let b = Bit(true); // Evaluates to an ON bit
let b2 = Bit::from(1_u8); // Evaluates to an ON bit
```
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Bit(bool);

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}

impl From<bool> for Bit {
    fn from(x: bool) -> Self {
        Self(x)
    }
}

impl From<u8> for Bit {
    fn from(x: u8) -> Self {
        Self(x != 0)
    }
}

// Misc Impls
impl Bit {
    pub fn flip(&mut self) {
        self.0 = !self.0;
    }

    pub fn one() -> Self {
        Self(true)
    }
    pub fn is_one(&self) -> bool {
        self.0
    }

    pub fn zero() -> Self {
        Self(false)
    }
    pub fn is_zero(&self) -> bool {
        !self.0
    }
}
