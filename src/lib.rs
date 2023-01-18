//! Crate for performing operations on bits and bytes

#![deny(missing_docs)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::{
    fmt,
    ops::{Add, BitAnd, BitOr, BitXor, Div, Mul, Neg, Not, Rem, Sub},
};

use num::{One, Zero};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Unsigned 1-bit Integer
/// Operations that would overflow, panic instead
pub struct u1(bool);

const b0: u1 = u1(false);
const b1: u1 = u1(true);

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
        // if let (b0, b0) | (b1, b0) = (self, rhs) {
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
        self.0.partial_cmp(&other.0)
    }
}

impl Zero for u1 {
    fn zero() -> Self {
        Self(false)
    }

    fn is_zero(&self) -> bool {
        !self.0
    }
}

impl One for u1 {
    fn one() -> Self {
        Self(true)
    }
}

/// A 3-bit floating point number.
/// The equation for the value of a float is (sign * mantissa * (2 ^ expontent))
#[derive(Clone, Copy, PartialOrd, Debug)]
pub struct f3 {
    /// Denotes whether the number is positive (0 means positive, 1 means negative)
    sign: u1,

    /// Modifies the magnitude of the number
    exponent: u1,

    /// Fractional part of the float
    mantissa: u1,
}

impl f3 {
    // CONSTANTS
    const ZERO: Self = Self::new(b0, b0, b0);
    const ONE: Self = Self::new(b0, b0, b1);
    const INFINITY: Self = Self::new(b0, b1, b0);
    const NAN: Self = Self::new(b0, b1, b1);
    const NEG_ZERO: Self = Self::new(b1, b0, b0);
    const NEG_ONE: Self = Self::new(b1, b0, b1);
    const NEG_INFINITY: Self = Self::new(b1, b1, b0);
    const NEG_NAN: Self = Self::new(b1, b1, b1);

    const fn new(sign: u1, exponent: u1, mantissa: u1) -> Self {
        Self {
            sign,
            exponent,
            mantissa,
        }
    }
}

impl Add for f3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let x: f32 = self.into();
        let y: f32 = rhs.into();
        (x + y).into()
    }
}

impl Sub for f3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        (f32::from(self) - f32::from(rhs)).into()
    }
}

impl Mul for f3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        (f32::from(self) * f32::from(rhs)).into()
    }
}

impl Div for f3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        (f32::from(self) / f32::from(rhs)).into()
    }
}

impl Rem for f3 {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        (f32::from(self) % f32::from(rhs)).into()
    }
}

impl One for f3 {
    fn one() -> Self {
        Self {
            sign: b0,
            exponent: b0,
            mantissa: b1,
        }
    }
}

impl Zero for f3 {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self == &Self::ZERO
    }
}

impl Neg for f3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let sign = !self.sign;
        let exponent = self.exponent;
        let mantissa = self.mantissa;

        Self::new(sign, exponent, mantissa)
    }
}

impl PartialEq for f3 {
    fn eq(&self, other: &Self) -> bool {
        // nan != nan
        if [self, other] == [&Self::NAN; 2] {
            false
        // +- 0 == 0
        } else if self.is_zero() && other.is_zero() {
            true
        // 1 == 1
        } else if [self, other] == [&Self::one(); 2] {
            true
        // -1 == -1
        } else if [self, other] == [&-Self::one(); 2] {
            true
        // -inf == -inf
        } else if [self, other] == [&Self::NEG_INFINITY; 2] {
            true
        // inf = inf
        } else if [self, other] == [&Self::INFINITY; 2] {
            true
        } else {
            false
        }
    }
}

impl From<f32> for f3 {
    fn from(value: f32) -> Self {
        // INFINITY
        if value == f32::INFINITY {
            Self::INFINITY
        }

        // -INFINITY
        else if value == f32::NEG_INFINITY {
            Self::NEG_INFINITY
        }

        // 0
        else if value == 0. {
            Self::ZERO
        }

        // -0
        else if value == -0. {
            Self::NEG_ZERO
        }

        // 1
        else if value == 1. {
            Self::ONE
        }

        // -1
        else if value == -1. {
            Self::NEG_ONE
        }

        // > +1 (INFINITY)
        else if value > 1. {
            Self::INFINITY
        }

        // < -1 (-INFINITY)
        else if value < -1. {
            Self::NEG_INFINITY
        }

        // NAN
        else if value.is_sign_positive() {
            Self::NAN
        }

        // -NAN
        else {
            Self::NEG_NAN
        }
    }
}

impl From<f3> for f32 {
    fn from(val: f3) -> Self {
        if val == f3::ZERO {
            0.
        } else if val == f3::NEG_ZERO {
            -0.
        } else if val == f3::ONE {
            1.
        } else if val == f3::NEG_ONE {
            -1.
        } else if val == f3::INFINITY {
            f32::INFINITY
        } else if val == f3::NEG_INFINITY {
            f32::NEG_INFINITY
        } else if val == f3::NEG_NAN {
            -f32::NAN
        } else {
            f32::NAN
        }
    }
}
