//! Crate for performing operations on bits and bytes

#![deny(missing_docs, clippy::needless_bool)]
#![allow(non_upper_case_globals, non_camel_case_types)]

mod f3;
mod u1;

mod byte {
    use crate::{u1::u1, AsBits};
    use std::ops::Index;
    /// u8 alias that can be indexed to get a specific bit
    #[derive(Copy, Clone, Debug)]
    struct Byte([u1; 8]);

    impl Index<usize> for Byte {
        type Output = u1;
        fn index(&self, index: usize) -> &Self::Output {
            self.0.index(index)
        }
    }

    impl From<u8> for Byte {
        fn from(value: u8) -> Self {
            Self(value.as_bits())
        }
    }
}

/// Gives access to a function that returns the bits of a number
pub trait AsBits {
    /// The number of bits necessary to represent this type        
    const BITS_TO_REPR: usize;

    /// MUST BE `type R = [u1; Self::R]`
    ///
    /// Sadly there isn't a nicer way to do this that I'm aware of :/
    type R;

    /// Returns the bits that make up `self`
    fn as_bits(&self) -> Self::R;
}

impl AsBits for u8 {
    const BITS_TO_REPR: usize = 8;

    type R = [crate::u1::u1; Self::BITS_TO_REPR];

    fn as_bits(&self) -> Self::R {
        [
            u1::u1(self & 128 > 0),
            u1::u1(self & 64 > 0),
            u1::u1(self & 32 > 0),
            u1::u1(self & 16 > 0),
            u1::u1(self & 8 > 0),
            u1::u1(self & 4 > 0),
            u1::u1(self & 2 > 0),
            u1::u1(self & 1 > 0),
        ]
    }
}
