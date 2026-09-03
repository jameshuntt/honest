//! Big integers that scrub their memory when dropped.
//!
//! `num-bigint` values live on the heap and forget nothing on their own.
//! [`SecureBigUint`] and [`SecureBigInt`] overwrite their limbs on
//! [`Zeroize`] and are meant to be held inside a [`SecretBox`], which
//! zeroizes on drop and only hands the value out through `expose_secret`.

use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
use secrecy::{ExposeSecret, SecretBox};
use zeroize::Zeroize;

use crate::modular;

/// An unsigned big integer whose limbs are overwritten on zeroize.
#[derive(Clone, PartialEq, Eq)]
pub struct SecureBigUint(pub BigUint);

impl Zeroize for SecureBigUint {
    fn zeroize(&mut self) {
        let mut limbs = core::mem::take(&mut self.0).to_u32_digits();
        limbs.zeroize();
        self.0 = BigUint::zero();
    }
}

/// A signed big integer whose limbs are overwritten on zeroize.
#[derive(Clone, PartialEq, Eq)]
pub struct SecureBigInt(pub BigInt);

impl Zeroize for SecureBigInt {
    fn zeroize(&mut self) {
        let (_, mut limbs) = core::mem::take(&mut self.0).to_u32_digits();
        limbs.zeroize();
        self.0 = BigInt::zero();
    }
}

/// A [`SecureBigUint`] held behind `secrecy`: zeroized on drop, read only
/// through `expose_secret`.
pub type SecretBigUint = SecretBox<SecureBigUint>;

/// A [`SecureBigInt`] held behind `secrecy`.
pub type SecretBigInt = SecretBox<SecureBigInt>;

/// Wrap a value as a secret.
pub fn secret_biguint(v: BigUint) -> SecretBigUint {
    SecretBox::new(Box::new(SecureBigUint(v)))
}

/// [`modular::modinv`] of a secret, with the result kept secret.
pub fn modinv(a: &SecretBigUint, m: &BigUint) -> Option<SecretBigUint> {
    modular::modinv(&a.expose_secret().0, m).map(secret_biguint)
}

/// [`modular::evaluate_mod`] with secret coefficients. The result is a share,
/// not the secret, so it is returned in the open.
pub fn evaluate_mod(coeffs: &[SecretBigUint], x: &BigUint, m: &BigUint) -> BigUint {
    coeffs
        .iter()
        .rev()
        .fold(BigUint::zero(), |acc, c| (acc * x + &c.expose_secret().0) % m)
}
