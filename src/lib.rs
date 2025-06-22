//! Crate for performing operations on bits and bytes

#![deny(missing_docs, clippy::needless_bool)]
#![allow(non_upper_case_globals, non_camel_case_types)]

pub mod bit;
pub mod f3;
pub mod nibble;

/// Returns the bits that make up a `u8`
pub fn bits_of_u8(n: u8) -> [bit::Bit; 8] {
    std::array::from_fn(|i| bit::Bit::from(n & (1 << i) > 0))
}

/// Returns a byte (`u8`) constructed from bits (`u1`s)
pub fn u8_from_bits(bits: [bit::Bit; 8]) -> u8 {
    bits.into_iter()
        .map(u8::from)
        .enumerate()
        .map(|(i, n)| n << i)
        .sum()
}

// Returns `num` with its `n`'th bit set to one
fn set_nth_bit(num: u8, n: u8) -> u8 {
    num | (1 << n)
}

#[test]
fn set_nth_bit_works() {
    assert_eq!(set_nth_bit(0, 7), 255);
}
