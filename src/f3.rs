//! Everything related to the f3 struct

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num::{One, Zero};

use crate::u1::{b0, b1, u1};

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
        (f32::from(self) + f32::from(rhs)).into()
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
        [
            [f3::ONE; 2],
            [f3::NEG_ONE; 2],
            [f3::ZERO; 2],
            [f3::NEG_ZERO; 2],
            [f3::INFINITY; 2],
            [f3::NEG_INFINITY; 2],
        ]
        .contains(&[*self, *other])
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
