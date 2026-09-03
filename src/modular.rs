//! Modular arithmetic over arbitrary-size integers, for schemes that work in
//! a prime field instead of GF(2^8).
//!
//! `num-bigint` operations are not constant-time; use these where the
//! operands are not secret, or accept that timing may leak their size.

use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};

/// Extended Euclid: `(g, s, t)` with `g = gcd(a, b)` and `a·s + b·t = g`.
pub fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();
    let mut old_t = BigInt::zero();
    let mut t = BigInt::one();
    while !r.is_zero() {
        let q = &old_r / &r;
        let q_int = BigInt::from(q.clone());
        let next_r = &old_r - &q * &r;
        old_r = core::mem::replace(&mut r, next_r);
        let next_s = &old_s - &q_int * &s;
        old_s = core::mem::replace(&mut s, next_s);
        let next_t = &old_t - &q_int * &t;
        old_t = core::mem::replace(&mut t, next_t);
    }
    (old_r, old_s, old_t)
}

/// `a⁻¹ mod m`, or `None` when `a` and `m` are not coprime (including `m ≤ 1`).
pub fn modinv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    if *m <= BigUint::one() {
        return None;
    }
    let (g, s, _) = extended_gcd(a, m);
    if !g.is_one() {
        return None;
    }
    let m_int = m.to_bigint()?;
    let reduced = ((s % &m_int) + &m_int) % &m_int;
    reduced.to_biguint()
}

/// `p(x) mod m` by Horner's rule; coefficients lowest degree first.
pub fn evaluate_mod(coeffs: &[BigUint], x: &BigUint, m: &BigUint) -> BigUint {
    coeffs
        .iter()
        .rev()
        .fold(BigUint::zero(), |acc, c| (acc * x + c) % m)
}
