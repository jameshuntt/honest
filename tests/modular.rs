//! The big-integer helpers behind the `bigint` feature.

#![cfg(feature = "bigint")]

use honest::modular::{evaluate_mod, extended_gcd, modinv};
use honest::secure::{self, secret_biguint, SecureBigInt, SecureBigUint};
use num_bigint::{BigInt, BigUint};
use secrecy::ExposeSecret;
use zeroize::Zeroize;

fn big(v: u64) -> BigUint {
    BigUint::from(v)
}

#[test]
fn extended_gcd_bezout_identity() {
    for (a, b) in [(240u64, 46u64), (17, 5), (1, 1), (0, 9), (100, 75)] {
        let (g, s, t) = extended_gcd(&big(a), &big(b));
        assert_eq!(BigInt::from(g.clone()), BigInt::from(a) * s + BigInt::from(b) * t, "{a} {b}");
        let expected = {
            let (mut x, mut y) = (a, b);
            while y != 0 {
                let r = x % y;
                x = y;
                y = r;
            }
            x
        };
        assert_eq!(g, big(expected));
    }
}

#[test]
fn modular_inverse_exists_exactly_when_coprime() {
    for m in [11u64, 97, 12, 100, 65537] {
        for a in 1..m.min(200) {
            let inv = modinv(&big(a), &big(m));
            let coprime = {
                let (mut x, mut y) = (a, m);
                while y != 0 {
                    let r = x % y;
                    x = y;
                    y = r;
                }
                x == 1
            };
            assert_eq!(inv.is_some(), coprime, "{a} mod {m}");
            if let Some(inv) = inv {
                assert_eq!((big(a) * inv) % big(m), big(1));
            }
        }
    }
    assert_eq!(modinv(&big(3), &big(1)), None);
    assert_eq!(modinv(&big(3), &big(0)), None);
}

#[test]
fn horner_mod_prime() {
    // 1 + 2x + 3x^2 at x = 2 mod 97 = 17
    let coeffs = [big(1), big(2), big(3)];
    assert_eq!(evaluate_mod(&coeffs, &big(2), &big(97)), big(17));
    assert_eq!(evaluate_mod(&[], &big(2), &big(97)), big(0));
    // reduction happens: 1 + 2·50 + 3·2500 = 7601 = 78·97 + 35
    assert_eq!(evaluate_mod(&coeffs, &big(50), &big(97)), big(35));
}

#[test]
fn secret_wrappers_scrub_and_still_compute() {
    let mut s = SecureBigUint(big(1234567890));
    s.zeroize();
    assert_eq!(s.0, big(0));
    assert!(s.0.to_u32_digits().is_empty());

    let mut i = SecureBigInt(BigInt::from(-99));
    i.zeroize();
    assert_eq!(i.0, BigInt::from(0));

    let a = secret_biguint(big(3));
    let inv = secure::modinv(&a, &big(11)).expect("3 is invertible mod 11");
    assert_eq!(inv.expose_secret().0, big(4));

    let coeffs = [secret_biguint(big(1)), secret_biguint(big(2)), secret_biguint(big(3))];
    assert_eq!(secure::evaluate_mod(&coeffs, &big(2), &big(97)), big(17));
}
