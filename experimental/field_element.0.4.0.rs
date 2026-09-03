use creusot_contracts::{ensures, pure};
use zeroize::Zeroize;
use std::ops::{Add, Sub, Mul, Div};

use crate::galois_fields::{gf256_exp, gf256_mul_fast};

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
        // Here you call your verified gf256_mul_fast or CT version
        FieldElement(gf256_mul_fast(self.0, rhs.0))
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

impl FieldElement {
    pub fn inv(self) -> Self {
        // Your existing Fermat implementation: a^(255-2)
        // This is safe because 0 is handled in the div trait or here
        if self.0 == 0 { panic!("0 has no inverse"); }
        FieldElement(gf256_exp(self.0, 254))
    }
}

impl From<u8> for FieldElement {
    fn from(b: u8) -> Self {
        FieldElement(b)
    }
}

// // Usage:
// let x = FieldElement::from(5);
// let y = FieldElement::from(10);
// let z = x * y;



// impl std::ops::Mul for FieldElement {
//     type Output = Self;
// 
//     fn mul(self, rhs: Self) -> Self {
//         let mut a = self.0;
//         let mut b = rhs.0;
//         let mut p = 0u8; // The product
// 
//         for _ in 0..8 {
//             if (b & 1) != 0 {
//                 p ^= a; // Galois Addition (XOR)
//             }
//             let hi_bit_set = (a & 0x80) != 0;
//             a <<= 1;
//             if hi_bit_set {
//                 a ^= 0x1B; // XOR with AES Polynomial (x^8 + x^4 + x^3 + x + 1)
//             }
//             b >>= 1;
//         }
//         FieldElement(p)
//     }
// }

use std::ops::BitXor;

impl BitXor for FieldElement {
    type Output = Self;

    // In Galois Field 2^8, XOR is exactly what "Addition" means.
    #[pure]
    #[ensures(result.0 == self.0 ^ rhs.0)]
    fn bitxor(self, rhs: Self) -> Self::Output {
        FieldElement(self.0 ^ rhs.0)
    }
}