use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{One, Zero};
use secrecy::{ExposeSecret, SecretBox};

use crate::{
    polynomial::extended_gcd::extended_gcd,
    types::secure_types::{
        SecretBigUint,
        SecureBigUint
    }
};

/// Computes the modular inverse of `a` modulo `m` using the Extended Euclidean Algorithm.
pub fn modinv(
    a: &BigUint,
    m: &BigUint
) -> Option<BigUint> {
    let (gcd, x, _) = extended_gcd(a, m);

    if gcd != BigUint::one() {
        return None;
    }

    let m_bigint = m.to_bigint().unwrap();
    let result = (x % &m_bigint + &m_bigint) % &m_bigint;

    Some(result.to_biguint().unwrap())
}

pub fn secure_modinv(
    secret: SecretBigUint,
    m: &BigUint
) -> Option<SecretBigUint> {
    let (gcd, x, _) = extended_gcd(
        &secret.expose_secret().0,
        m
    );

    if gcd != BigUint::one() {
        return None;
    }

    let m_bigint = m.to_bigint().unwrap();
    let inv = (x % &m_bigint + &m_bigint) % &m_bigint;
    
    Some(SecretBox::new(Box::new(
        SecureBigUint(inv.to_biguint().unwrap())
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_modinv_basic() {
        let a = 3u32.to_biguint().unwrap();
        let m = 11u32.to_biguint().unwrap();
        let inv = modinv(&a, &m).unwrap();
        assert_eq!((&a * &inv) % &m, 1u32.to_biguint().unwrap());
    }

    #[test]
    fn test_modinv_non_invertible() {
        let a = 6u32.to_biguint().unwrap(); // gcd(6, 12) = 6 ≠ 1
        let m = 12u32.to_biguint().unwrap();
        assert!(modinv(&a, &m).is_none());
    }

    #[test]
    fn test_secure_modinv_basic() {
        let a = 3u32.to_biguint().unwrap();
        let m = 11u32.to_biguint().unwrap();
        let secret = SecretBox::new(Box::new(SecureBigUint(a.clone())));
        let inv = secure_modinv(secret, &m).unwrap();
        assert_eq!((&a * &inv.expose_secret().0) % &m, 1u32.to_biguint().unwrap());
    }
}



// /// Computes the modular inverse of `a` modulo `m` using the Extended Euclidean Algorithm.
// pub(crate) fn modinv(a: &BigUint, m: &BigUint) -> Option<BigUint> {
//     let (gcd, x, _) = extended_gcd(a, m);
//     if gcd != BigUint::one() {
//         None
//     } else {
//         Some((x % m.to_bigint().unwrap() + m.to_bigint().unwrap()) % m.to_bigint().unwrap())
//             .map(|res| res.to_biguint().unwrap())
//     }
// }
// 

pub fn modinverse(
    a: &BigUint,
    m: &BigUint
) -> Option<BigUint> {
    let (mut mn, mut xy) = (
        (m.clone(), a.clone()),
        (BigInt::zero(), BigInt::one())
    );

    while mn.1 != BigUint::zero() {
        let quotient = &mn.0 / &mn.1;
        mn = (mn.1.clone(), &mn.0 - &quotient * &mn.1);
        xy = (xy.1.clone(), &xy.0 - &quotient.to_bigint().unwrap() * &xy.1);
    }

    if mn.0 != BigUint::one() {
        return None;
    }

    Some((xy.0 % m.to_bigint().unwrap() + m.to_bigint().unwrap()) % m.to_bigint().unwrap())
        .map(|v| v.to_biguint().unwrap())
}
