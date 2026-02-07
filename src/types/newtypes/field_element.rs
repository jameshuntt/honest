use zeroize::Zeroize;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Copy, Clone, PartialEq, Eq, Default, Zeroize)]
pub struct FieldElement(pub u8);

// Now we implement the traits so we can use +, -, *, /
impl Add for FieldElement {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FieldElement(self.0 ^ rhs.0)
    }
}

impl Sub for FieldElement {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.add(rhs) // Subtraction is Addition in GF(2^8)
    }
}

impl Mul for FieldElement {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut a = self.0;
        let mut b = rhs.0;
        let mut p = 0u8; // The product

        for _ in 0..8 {
            if (b & 1) != 0 {
                p ^= a; // Galois Addition (XOR)
            }
            let hi_bit_set = (a & 0x80) != 0;
            a <<= 1;
            if hi_bit_set {
                a ^= 0x1B; // XOR with AES Polynomial (x^8 + x^4 + x^3 + x + 1)
            }
            b >>= 1;
        }
        FieldElement(p)
    }
}


impl Div for FieldElement {
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self {
        if rhs.0 == 0 {
            // In a professional crate, you might panic here 
            // or return a 'Zero' element depending on the spec.
            panic!("Division by zero in GF(256)");
        }
        // a / b is just a * inv(b)
        self * rhs.inv()
    }
}

use crate::galois_fields::{call_exp};
impl FieldElement {
    pub fn inv(self) -> Self {
        // Fermat implementation: a^(255-2)
        // This is safe because 0 is handled in the div trait or here
        if self.0 == 0 { panic!("0 has no inverse"); }
        FieldElement(call_exp(self.0, 254))
    }
}

use std::ops::BitXor;
impl BitXor for FieldElement {
    type Output = Self;

    // In Galois Field 2^8, XOR is exactly what "Addition" means.
    fn bitxor(self, rhs: Self) -> Self::Output {
        FieldElement(self.0 ^ rhs.0)
    }
}


use crate::traits::GaloisField;
use crate::galois_fields::gf256_mul_ct;
impl GaloisField for FieldElement {
    fn run_add(self, other: Self) -> Self { self.add(other) }
    fn run_sub(self, other: Self) -> Self { self.sub(other) }
    fn run_mul(self, other: Self) -> Self {
        FieldElement(gf256_mul_ct(self.0, other.0))
    }
    fn run_inv(self) -> Self { self.inv() }
    const ZERO: Self = FieldElement(0);
    const ONE: Self = FieldElement(1);
}



impl From<u8> for FieldElement {
    fn from(b: u8) -> Self {
        FieldElement(b)
    }
}