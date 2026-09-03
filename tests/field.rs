//! The field axioms, checked against every element where that is cheap and
//! against a wide grid where it is not, plus the published AES values.

use honest::{Field, Gf256};

fn all() -> impl Iterator<Item = Gf256> {
    (0u8..=255).map(Gf256)
}

#[test]
fn addition_is_xor_commutative_and_self_inverse() {
    for a in all() {
        for b in all() {
            assert_eq!(a + b, Gf256(a.0 ^ b.0));
            assert_eq!(a + b, b + a);
            assert_eq!(a - b, a + b);
            assert_eq!((a + b) + b, a);
            assert_eq!(-a, a);
        }
        assert_eq!(a + Gf256::ZERO, a);
    }
}

#[test]
fn multiplication_is_commutative_with_identities() {
    for a in all() {
        assert_eq!(a * Gf256::ONE, a);
        assert_eq!(a * Gf256::ZERO, Gf256::ZERO);
        for b in all() {
            assert_eq!(a * b, b * a);
        }
    }
}

#[test]
fn multiplication_is_associative_and_distributive_over_a_grid() {
    // every 7th element gives 37 values; 37^3 triples of two multiplications each
    let grid: Vec<Gf256> = (0u8..=255).step_by(7).map(Gf256).collect();
    for &a in &grid {
        for &b in &grid {
            for &c in &grid {
                assert_eq!((a * b) * c, a * (b * c), "assoc {a} {b} {c}");
                assert_eq!(a * (b + c), a * b + a * c, "distrib {a} {b} {c}");
            }
        }
    }
}

#[test]
fn every_nonzero_element_has_an_inverse_and_zero_does_not() {
    assert_eq!(Gf256::ZERO.inv(), None);
    assert_eq!(Gf256::ZERO.inv_or_zero(), Gf256::ZERO);
    assert_eq!(Gf256::ZERO.div(Gf256::ONE), Some(Gf256::ZERO));
    assert_eq!(Gf256::ONE.div(Gf256::ZERO), None);
    for a in all().skip(1) {
        let inv = a.inv().expect("non-zero has an inverse");
        assert_eq!(a * inv, Gf256::ONE, "{a}");
        assert_eq!(a.inv_or_zero(), inv);
        assert_eq!(inv.inv(), Some(a));
        for b in all().skip(1) {
            let q = a.div(b).unwrap();
            assert_eq!(q * b, a, "{a} / {b}");
        }
    }
}

#[test]
fn fermat_holds_and_pow_matches_repeated_multiplication() {
    for a in all().skip(1) {
        assert_eq!(a.pow(255), Gf256::ONE, "{a}^255");
        assert_eq!(a.pow(254), a.inv().unwrap());
    }
    for a in all() {
        assert_eq!(a.pow(0), Gf256::ONE);
        assert_eq!(a.pow(1), a);
        let mut acc = Gf256::ONE;
        for e in 0..=20u16 {
            assert_eq!(a.pow(e), acc, "{a}^{e}");
            acc *= a;
        }
    }
    assert_eq!(Gf256::ZERO.pow(0), Gf256::ONE, "0^0 is 1 by the usual convention");
}

#[test]
fn x_itself_has_order_51_so_it_is_not_a_generator() {
    let mut x = Gf256::ONE;
    for _ in 0..51 {
        x *= Gf256(2);
    }
    assert_eq!(x, Gf256::ONE);
}

#[test]
fn the_generator_visits_every_nonzero_element_once() {
    let mut seen = [false; 256];
    let mut x = Gf256::ONE;
    for _ in 0..255 {
        assert!(!seen[x.0 as usize], "cycle shorter than 255 at {x}");
        seen[x.0 as usize] = true;
        x *= Gf256::GENERATOR;
    }
    assert_eq!(x, Gf256::ONE);
    assert!(!seen[0]);
    assert!(seen[1..].iter().all(|&s| s));
}

#[test]
fn fips_197_values() {
    // FIPS-197 section 4.2: {57} • {83} = {c1}
    assert_eq!(Gf256(0x57) * Gf256(0x83), Gf256(0xc1));
    // section 4.2.1: {57} • {13} = {fe}
    assert_eq!(Gf256(0x57) * Gf256(0x13), Gf256(0xfe));
    // xtime examples: {57}·{02} = {ae}, {ae}·{02} = {47}, {47}·{02} = {8e}, {8e}·{02} = {07}
    assert_eq!(Gf256(0x57) * Gf256(0x02), Gf256(0xae));
    assert_eq!(Gf256(0xae) * Gf256(0x02), Gf256(0x47));
    assert_eq!(Gf256(0x47) * Gf256(0x02), Gf256(0x8e));
    assert_eq!(Gf256(0x8e) * Gf256(0x02), Gf256(0x07));
    // the S-box example: inverse of {53} is {ca}
    assert_eq!(Gf256(0x53).inv(), Some(Gf256(0xca)));
}

#[test]
fn sums_products_and_conversions() {
    let v = [Gf256(1), Gf256(2), Gf256(3)];
    assert_eq!(v.iter().copied().sum::<Gf256>(), Gf256(1 ^ 2 ^ 3));
    assert_eq!(v.iter().copied().product::<Gf256>(), Gf256(1) * Gf256(2) * Gf256(3));
    assert_eq!(u8::from(Gf256::from(0x9a)), 0x9a);
    assert_eq!(format!("{}", Gf256(0x0f)), "0x0f");
    assert_eq!(format!("{:?}", Gf256(0x0f)), "Gf256(0x0f)");
    let mut a = Gf256(5);
    a += Gf256(3);
    a *= Gf256(2);
    a -= Gf256(1);
    assert_eq!(a, ((Gf256(5) + Gf256(3)) * Gf256(2)) - Gf256(1));
}

#[test]
fn the_field_trait_agrees_with_the_inherent_methods() {
    fn generic<F: Field>(a: F, b: F) -> (F, F, F, Option<F>) {
        (a.add(b), a.sub(b), a.mul(b), a.inv())
    }
    for a in all() {
        for b in [Gf256(0), Gf256(1), Gf256(0x53), Gf256(0xff)] {
            assert_eq!(generic(a, b), (a + b, a - b, a * b, a.inv()));
        }
    }
    assert_eq!(<Gf256 as Field>::ZERO, Gf256::ZERO);
    assert_eq!(<Gf256 as Field>::ONE, Gf256::ONE);
}

#[test]
fn elements_zeroize() {
    use zeroize::Zeroize;
    let mut a = Gf256(0xab);
    a.zeroize();
    assert_eq!(a, Gf256::ZERO);
}
