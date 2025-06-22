//! Everything related to the u1 struct

use std::{
    cmp::Ordering,
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// Represents one bit of information.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Bit {
    #[default]
    /// Represents 0 or false.
    Zero = 0,

    /// Represents 1 or true.
    One = 1,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bit::Zero => write!(f, "0"),
            Bit::One => write!(f, "1"),
        }
    }
}

impl Bit {
    /// Returns `true` if the bit is [`Zero`].
    ///
    /// [`Zero`]: Bit::Zero
    #[must_use]
    pub fn is_zero(&self) -> bool {
        matches!(self, Self::Zero)
    }

    /// Returns `true` if the bit is [`One`].
    ///
    /// [`One`]: Bit::One
    #[must_use]
    pub fn is_one(&self) -> bool {
        matches!(self, Self::One)
    }

    /// Returns 1 if `self` and `other` aren't both 1
    pub fn nand(self, other: Self) -> Self {
        !(self & other)
    }

    /// Adds two bits, with a carry-in and a carry-out bit.
    /// This allows adds to be chained together without panicking.
    pub fn add(self, other: Self, carry_in: Self) -> (Self, Self) {
        let a = self ^ other;
        let b = self & other;
        let c = a & carry_in;
        let sum = a ^ carry_in;
        let carry_out = c | b;
        (sum, carry_out)
    }
}

impl Not for Bit {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}

impl BitAnd for Bit {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::One, Self::One) => Self::One,
            _ => Self::Zero,
        }
    }
}

impl BitAndAssign for Bit {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs;
    }
}

impl BitOr for Bit {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        (!self).nand(!rhs)
    }
}

impl BitOrAssign for Bit {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitXor for Bit {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self | rhs) & self.nand(rhs)
    }
}

impl BitXorAssign for Bit {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs;
    }
}

impl Ord for Bit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Bit::Zero, Bit::One) => Ordering::Less,
            (Bit::One, Bit::Zero) => Ordering::Greater,
            (Bit::Zero, Bit::Zero) => Ordering::Equal,
            (Bit::One, Bit::One) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Bit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        match value {
            true => Self::One,
            false => Self::Zero,
        }
    }
}

macro_rules! impl_from_bit_for_primint {
    ($($t: ty),*) => {
        $(
          impl From<Bit> for $t {
              fn from(value: Bit)->Self {
                  match value {
                      Bit::Zero => 0,
                      Bit::One => 1,
                  }
              }
          }
        )*
    };
}

impl_from_bit_for_primint!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
