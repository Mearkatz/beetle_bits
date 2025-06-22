//! Everything related to the nibble struct
use crate::bit::Bit;
use std::{
    fmt::Display,
    ops::{Not, Sub},
};

/// Half a byte, or 4 Bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Nibble([Bit; 4]);

impl From<Nibble> for i8 {
    fn from(value: Nibble) -> Self {
        let a: i8 = if value.0[0].is_one() { -8 } else { 0 };
        let b: i8 = if value.0[1].is_one() { 4 } else { 0 };
        let c: i8 = if value.0[2].is_one() { 2 } else { 0 };
        let d: i8 = if value.0[3].is_one() { 1 } else { 0 };
        a + b + c + d
    }
}

impl Not for Nibble {
    type Output = Self;

    /// Returns the Nibble with all its bits inverted
    fn not(self) -> Self::Output {
        Self(self.0.map(Not::not))
    }
}

impl Display for Nibble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", i8::from(*self))
    }
}

impl Nibble {
    /**
    Bitwise addition of two nibbles.
    I'm pretty sure this is unsigned addition.
    */
    pub fn add(self, other: Self, carry_in: Bit) -> (Self, Bit) {
        let (sum1, carry) = self.0[0].add(other.0[0], carry_in);
        let (sum2, carry) = self.0[1].add(other.0[1], carry);
        let (sum3, carry) = self.0[2].add(other.0[2], carry);
        let (sum4, carry_out) = self.0[3].add(other.0[3], carry);
        let total_sum = Self([sum1, sum2, sum3, sum4]);
        (total_sum, carry_out)
    }
}

impl Sub for Nibble {
    type Output = (Self, Bit);

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(!rhs, Bit::One)
    }
}
