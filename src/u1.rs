//! Everything related to the u1 struct

use std::{
    fmt,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Not, Rem, Sub},
};

use num_traits::{One, Zero};

/// Unsigned 1-bit Integer (a bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct u1(pub bool);

/// A u1 representing zero
pub const b0: u1 = u1(false);

/// A u1 representing one
pub const b1: u1 = u1(true);

impl fmt::Display for u1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            write!(f, "1")
        } else {
            write!(f, "0")
        }
    }
}

impl BitAnd for u1 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitXor for u1 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitOr for u1 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for u1 {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl Add for u1 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (b0, b0) => b0,
            (b1, b1) => panic!("u1_1 + u1_1 overflow"),
            _ => b1,
        }
    }
}

impl Sub for u1 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (b0, b0) | (b1, b1) => b0,
            (b1, b0) => b1,
            (b0, b1) => panic!("u1_1 - u1_1 underflow"),
        }
    }
}

impl Mul for u1 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // 1 * 1 == 1
        if let (b1, b1) = (self, rhs) {
            b1
        // (0 * x) | (x * 0) == 0
        } else {
            b0
        }
    }
}

impl Div for u1 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (_, b0) => panic!("Attempt to divide a u1 by zero, which is undefined"),
            (b0, b1) => b0,
            (b1, b1) => b1,
        }
    }
}

impl Rem for u1 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        if rhs.is_zero() {
            panic!("Attempt to modulo a u1 by zero, which is undefined")
        } else {
            b0
        }
    }
}

impl Ord for u1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for u1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Zero for u1 {
    fn zero() -> Self {
        b0
    }

    fn is_zero(&self) -> bool {
        !self.0
    }
}

impl One for u1 {
    fn one() -> Self {
        b1
    }
}

impl From<bool> for u1 {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<u1> for u8 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for u16 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for u32 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for u64 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for u128 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for usize {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for i8 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for i16 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for i32 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for i64 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for i128 {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}

impl From<u1> for isize {
    fn from(value: u1) -> Self {
        value.0.into()
    }
}
