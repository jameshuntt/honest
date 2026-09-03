//! GF(2^8) with the AES reduction polynomial `x^8 + x^4 + x^3 + x + 1`.
//!
//! Every operation runs the same instructions regardless of the operand
//! values: addition is a single XOR, multiplication is eight rounds of
//! mask-and-shift, and exponentiation walks a fixed number of bits of an
//! exponent that is never secret (the field's only interesting exponent is
//! the constant `254`). There are no lookup tables, so there is no
//! cache-timing channel either.

use core::fmt;
use core::ops::{Add, AddAssign, BitXor, Mul, MulAssign, Neg, Sub, SubAssign};
use zeroize::Zeroize;

/// The reduction polynomial `x^8 + x^4 + x^3 + x + 1` without its top bit.
pub const REDUCTION: u8 = 0x1B;

/// A field: the operations a scheme needs, with the same shape for every
/// implementation so a polynomial routine can be written once.
pub trait Field: Copy + Eq {
    /// The additive identity.
    const ZERO: Self;
    /// The multiplicative identity.
    const ONE: Self;
    /// `self + rhs`.
    fn add(self, rhs: Self) -> Self;
    /// `self - rhs`.
    fn sub(self, rhs: Self) -> Self;
    /// `self * rhs`.
    fn mul(self, rhs: Self) -> Self;
    /// The multiplicative inverse, `None` for zero.
    fn inv(self) -> Option<Self>;
}

/// An element of GF(2^8).
///
/// The wrapped byte is the polynomial's coefficients, bit `i` for `x^i`.
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Zeroize)]
pub struct Gf256(pub u8);

impl Gf256 {
    /// The additive identity.
    pub const ZERO: Self = Self(0);
    /// The multiplicative identity.
    pub const ONE: Self = Self(1);
    /// The element `x + 1` (0x03), a generator of the multiplicative group:
    /// its powers visit every non-zero element. (`x` itself is not one; its
    /// order is 51.)
    pub const GENERATOR: Self = Self(3);

    /// The element with these coefficient bits.
    #[inline]
    pub const fn new(v: u8) -> Self {
        Self(v)
    }

    /// The coefficient bits.
    #[inline]
    pub const fn value(self) -> u8 {
        self.0
    }

    /// Whether this is zero.
    #[inline]
    pub const fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// `self + rhs`: XOR.
    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }

    /// `self - rhs`: the same XOR, since every element is its own negative.
    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        self.add(rhs)
    }

    /// `self * rhs` in eight fixed rounds. Each round conditionally adds `a`
    /// through a mask derived from the low bit of `b`, then multiplies `a` by
    /// `x` with the reduction applied through a mask derived from its top bit.
    /// No branch depends on either operand.
    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        let mut a = self.0;
        let mut b = rhs.0;
        let mut p = 0u8;
        let mut round = 0;
        while round < 8 {
            let include_mask = 0u8.wrapping_sub(b & 1);
            p ^= a & include_mask;
            let hi_mask = 0u8.wrapping_sub((a >> 7) & 1);
            a = (a << 1) ^ (REDUCTION & hi_mask);
            b >>= 1;
            round += 1;
        }
        Self(p)
    }

    /// `self * self`.
    #[inline]
    pub const fn square(self) -> Self {
        self.mul(self)
    }

    /// `self^exp`. Walks all sixteen bits of `exp` every time, so the running
    /// time depends only on the exponent's width, never on its value or on
    /// the base. Exponents are public in every use this crate is built for.
    #[inline]
    pub const fn pow(self, exp: u16) -> Self {
        let mut base = self;
        let mut acc = Self::ONE;
        let mut e = exp;
        let mut round = 0;
        while round < 16 {
            let take = 0u8.wrapping_sub((e & 1) as u8);
            let with = acc.mul(base);
            acc = Self((with.0 & take) | (acc.0 & !take));
            base = base.square();
            e >>= 1;
            round += 1;
        }
        acc
    }

    /// The multiplicative inverse, `a^254`, or `None` for zero.
    #[inline]
    pub const fn inv(self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(self.pow(254))
        }
    }

    /// The inverse, with zero mapping to zero: the convention the AES S-box
    /// uses. Computed as `a^254` for every input, so no branch on the value.
    #[inline]
    pub const fn inv_or_zero(self) -> Self {
        self.pow(254)
    }

    /// `self / rhs`, or `None` when `rhs` is zero.
    #[inline]
    pub const fn div(self, rhs: Self) -> Option<Self> {
        match rhs.inv() {
            Some(i) => Some(self.mul(i)),
            None => None,
        }
    }
}

impl Field for Gf256 {
    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Gf256::add(self, rhs)
    }
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Gf256::sub(self, rhs)
    }
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Gf256::mul(self, rhs)
    }
    #[inline]
    fn inv(self) -> Option<Self> {
        Gf256::inv(self)
    }
}

impl fmt::Debug for Gf256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Gf256(0x{:02x})", self.0)
    }
}

impl fmt::Display for Gf256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:02x}", self.0)
    }
}

impl From<u8> for Gf256 {
    #[inline]
    fn from(v: u8) -> Self {
        Self(v)
    }
}

impl From<Gf256> for u8 {
    #[inline]
    fn from(v: Gf256) -> Self {
        v.0
    }
}

impl Add for Gf256 {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Gf256::add(self, rhs)
    }
}

impl AddAssign for Gf256 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Gf256::add(*self, rhs);
    }
}

impl Sub for Gf256 {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Gf256::sub(self, rhs)
    }
}

impl SubAssign for Gf256 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Gf256::sub(*self, rhs);
    }
}

impl Mul for Gf256 {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Gf256::mul(self, rhs)
    }
}

impl MulAssign for Gf256 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = Gf256::mul(*self, rhs);
    }
}

impl BitXor for Gf256 {
    type Output = Self;
    #[inline]
    fn bitxor(self, rhs: Self) -> Self {
        Gf256::add(self, rhs)
    }
}

impl Neg for Gf256 {
    type Output = Self;
    /// Every element is its own negative in characteristic two.
    #[inline]
    fn neg(self) -> Self {
        self
    }
}

impl core::iter::Sum for Gf256 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, Gf256::add)
    }
}

impl core::iter::Product for Gf256 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, Gf256::mul)
    }
}
