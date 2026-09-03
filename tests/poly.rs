//! Evaluation and interpolation over GF(2^8).

use honest::{evaluate, interpolate_at, interpolate_at_zero, Gf256, PolyError};

/// A small deterministic generator so the round trips are reproducible.
struct Lcg(u64);

impl Lcg {
    fn next(&mut self) -> u8 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 56) as u8
    }
}

fn naive(coeffs: &[Gf256], x: Gf256) -> Gf256 {
    let mut acc = Gf256::ZERO;
    let mut power = Gf256::ONE;
    for &c in coeffs {
        acc += c * power;
        power *= x;
    }
    acc
}

#[test]
fn horner_matches_the_naive_sum() {
    let mut rng = Lcg(7);
    for degree in 0..12 {
        let coeffs: Vec<Gf256> = (0..=degree).map(|_| Gf256(rng.next())).collect();
        for x in (0u8..=255).map(Gf256) {
            assert_eq!(evaluate(&coeffs, x), naive(&coeffs, x));
        }
    }
    assert_eq!(evaluate(&[], Gf256(9)), Gf256::ZERO);
    assert_eq!(evaluate(&[Gf256(0x2a)], Gf256(9)), Gf256(0x2a));
}

#[test]
fn interpolation_recovers_the_polynomial_from_any_degree_plus_one_points() {
    let mut rng = Lcg(11);
    for degree in 0..10u8 {
        let coeffs: Vec<Gf256> = (0..=degree).map(|_| Gf256(rng.next())).collect();
        // distinct x's, skipping zero so the constant term is not handed over
        let points: Vec<(Gf256, Gf256)> = (1..=degree + 1)
            .map(|x| (Gf256(x.wrapping_mul(37).max(1)), Gf256::ZERO))
            .map(|(x, _)| (x, evaluate(&coeffs, x)))
            .collect();
        assert_eq!(interpolate_at_zero(&points), Ok(coeffs[0]), "degree {degree}");
        for probe in [Gf256(0x10), Gf256(0x80), Gf256(0xff)] {
            assert_eq!(interpolate_at(&points, probe), Ok(evaluate(&coeffs, probe)));
        }
    }
}

#[test]
fn fewer_points_than_the_degree_needs_give_a_different_answer() {
    // degree-2 polynomial, only two points: the interpolant is the line through them
    let coeffs = [Gf256(0x2a), Gf256(0x11), Gf256(0x07)];
    let points: Vec<(Gf256, Gf256)> = [1u8, 2].iter().map(|&x| (Gf256(x), evaluate(&coeffs, Gf256(x)))).collect();
    assert_ne!(interpolate_at_zero(&points), Ok(coeffs[0]));
}

#[test]
fn the_order_of_points_does_not_matter() {
    let coeffs = [Gf256(0xc3), Gf256(0x05), Gf256(0x99), Gf256(0x42)];
    let mut points: Vec<(Gf256, Gf256)> = [3u8, 9, 27, 81].iter().map(|&x| (Gf256(x), evaluate(&coeffs, Gf256(x)))).collect();
    let forward = interpolate_at_zero(&points).unwrap();
    points.reverse();
    assert_eq!(interpolate_at_zero(&points).unwrap(), forward);
    points.swap(0, 2);
    assert_eq!(interpolate_at_zero(&points).unwrap(), forward);
    assert_eq!(forward, Gf256(0xc3));
}

#[test]
fn refusals() {
    assert_eq!(interpolate_at_zero::<Gf256>(&[]), Err(PolyError::Empty));
    let dup = [(Gf256(1), Gf256(5)), (Gf256(2), Gf256(6)), (Gf256(1), Gf256(7))];
    assert_eq!(interpolate_at_zero(&dup), Err(PolyError::DuplicatePoint));
    assert_eq!(PolyError::Empty.to_string(), "no points to interpolate");
    assert_eq!(PolyError::DuplicatePoint.to_string(), "two points share an x coordinate");
}

#[test]
fn a_single_point_is_a_constant() {
    assert_eq!(interpolate_at_zero(&[(Gf256(5), Gf256(123))]), Ok(Gf256(123)));
    assert_eq!(interpolate_at(&[(Gf256(5), Gf256(123))], Gf256(200)), Ok(Gf256(123)));
}

#[test]
fn shares_add_like_the_polynomials_they_come_from() {
    // Shamir is linear: sharing s1 and s2 and adding the shares pointwise
    // gives shares of s1 + s2. This is the property proactive refresh relies on.
    let p = [Gf256(0x10), Gf256(0x33), Gf256(0x7e)];
    let q = [Gf256(0x0f), Gf256(0x02), Gf256(0xa1)];
    let xs = [Gf256(4), Gf256(8), Gf256(16)];
    let sum_shares: Vec<(Gf256, Gf256)> = xs.iter().map(|&x| (x, evaluate(&p, x) + evaluate(&q, x))).collect();
    assert_eq!(interpolate_at_zero(&sum_shares), Ok(p[0] + q[0]));
}

mod lagrange {
    use honest::poly::{interpolate_at_zero, lagrange_at_zero, PolyError};
    use honest::{evaluate, Gf256};

    #[test]
    fn the_basis_dot_product_is_the_value_at_zero() {
        let coeffs = [Gf256(0x53), Gf256(0xCA), Gf256(0x7E), Gf256(0x01)];
        let xs = [Gf256(1), Gf256(2), Gf256(3), Gf256(4)];
        let ys: Vec<Gf256> = xs.iter().map(|&x| evaluate(&coeffs, x)).collect();

        let mut basis = [Gf256::ZERO; 4];
        lagrange_at_zero(&xs, &mut basis).unwrap();
        let dot = basis.iter().zip(&ys).fold(Gf256::ZERO, |acc, (&b, &y)| acc.add(b.mul(y)));

        assert_eq!(dot, coeffs[0]);
        let points: Vec<(Gf256, Gf256)> = xs.iter().copied().zip(ys).collect();
        assert_eq!(dot, interpolate_at_zero(&points).unwrap());
    }

    #[test]
    fn the_basis_agrees_with_interpolation_for_every_pair_of_xs() {
        for a in 1u8..255 {
            for b in (a + 1)..=255u8 {
                let xs = [Gf256(a), Gf256(b)];
                let mut basis = [Gf256::ZERO; 2];
                lagrange_at_zero(&xs, &mut basis).unwrap();
                // a line through (a, ya) and (b, yb): pick two values and check
                let (ya, yb) = (Gf256(a ^ 0x5A), Gf256(b.wrapping_mul(3)));
                let dot = basis[0].mul(ya).add(basis[1].mul(yb));
                let via_points = interpolate_at_zero(&[(xs[0], ya), (xs[1], yb)]).unwrap();
                assert_eq!(dot, via_points, "xs = ({a}, {b})");            }
        }
    }

    #[test]
    fn the_basis_sums_to_one() {
        // Σ L_i(0) = 1 because the constant polynomial 1 interpolates itself.
        let xs = [Gf256(9), Gf256(30), Gf256(77), Gf256(200), Gf256(255)];
        let mut basis = [Gf256::ZERO; 5];
        lagrange_at_zero(&xs, &mut basis).unwrap();
        assert_eq!(basis.iter().fold(Gf256::ZERO, |acc, &b| acc.add(b)), Gf256::ONE);
    }

    #[test]
    fn refusals() {
        let mut out = [Gf256::ZERO; 2];
        assert_eq!(lagrange_at_zero::<Gf256>(&[], &mut []), Err(PolyError::Empty));
        assert_eq!(lagrange_at_zero(&[Gf256(1), Gf256(1)], &mut out), Err(PolyError::DuplicatePoint));
        assert_eq!(lagrange_at_zero(&[Gf256(1), Gf256(2), Gf256(3)], &mut out), Err(PolyError::LengthMismatch));
        assert_eq!(PolyError::LengthMismatch.to_string(), "output slice must have one entry per x coordinate");
    }
}
