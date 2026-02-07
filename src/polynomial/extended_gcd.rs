use num_bigint::{BigInt, BigUint};
use num_traits::Zero;
/// Extended Euclidean Algorithm to find the GCD and the coefficients.
#[must_use]
pub(crate) fn extended_gcd(
    a: &BigUint,
    b: &BigUint
) -> (BigUint, BigInt, BigInt) {
    let zero = BigUint::zero();
    let mut old_r = a.clone();
    let mut r = b.clone();
    let mut old_s = BigInt::from(1);
    let mut s = BigInt::from(0);
    let mut old_t = BigInt::from(0);
    let mut t = BigInt::from(1);

    while r != zero {
        let quotient = &old_r / &r;
        let temp_r = old_r.clone();
        old_r = r.clone();
        r = temp_r - &quotient * &r;

        let temp_s = old_s.clone();
        old_s = s.clone();
        s = temp_s - &BigInt::from(quotient.clone()) * &s;

        let temp_t = old_t.clone();
        old_t = t.clone();
        t = temp_t - &BigInt::from(quotient) * &t;
    }

    (old_r, old_s, old_t)
}


use secrecy::ExposeSecret;
use crate::types::secure_types::SecretBigUint;
#[must_use]
pub fn secure_extended_gcd(
    a: &SecretBigUint,
    b: &BigUint
) -> (BigUint, BigInt, BigInt) {
    let (gcd, x, y) = extended_gcd(
        &a.expose_secret().0,
        b
    );

    (gcd, x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_extended_gcd() {
        let a = 240u32.to_biguint().unwrap();
        let b = 46u32.to_biguint().unwrap();

        let (gcd, x, y) = extended_gcd(&a, &b);

        assert_eq!(gcd, 2u32.to_biguint().unwrap());
        let lhs = BigInt::from(a) * x + BigInt::from(b) * y;
        assert_eq!(lhs, BigInt::from(gcd));
    }
}
