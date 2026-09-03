use creusot_contracts::{ensures, requires};

use crate::galois_fields::{gf256_add, gf256_div, gf256_mul, gf256_sub};

// use crate::helpers::gf256_add;

// use super::{gf256_div, gf256_mul, gf256_sub};

pub(crate) fn lagrange_interpolate_at_zero(
    shares: &[(u8, u8)]
) -> u8 {
    let mut secret = 0u8;

    for (i, &(xi, yi)) in shares.iter().enumerate() {
        let mut num = 1u8;
        let mut den = 1u8;

        for (j, &(xj, _)) in shares.iter().enumerate() {
            if i == j {
                continue;
            }
            num = gf256_mul(num, xj);
            den = gf256_mul(den, gf256_sub(xj, xi));
        }

        let li = gf256_div(num, den);
        secret ^= gf256_mul(li, yi);
    }

    secret
}
#[requires(shares.len() > 0)]
#[requires(forall<i: Int, j: Int> 0 <= i && i < j && j < shares.len() ==> shares[i].0 != shares[j].0)]
#[ensures(result == lagrange_logic_spec(shares, 0))]
pub(crate) fn lagrange_interpolate_at_zero_b(shares: &[(u8, u8)]) -> u8 {
    let mut secret = 0u8;

    for i in 0..shares.len() {
        let (xi, yi) = shares[i];
        let mut num = 1u8;
        let mut den = 1u8;

        for j in 0..shares.len() {
            if i == j { continue; }
            let (xj, _) = shares[j];
            
            num = gf256_mul(num, xj);
            // In GF(256), sub is XOR, but using the function is better for the prover
            den = gf256_mul(den, gf256_sub(xj, xi)); 
        }

        // Creusot now knows den != 0 because shares are unique
        let li = gf256_div(num, den);
        secret = gf256_add(secret, gf256_mul(li, yi));
    }

    secret
}

// #[requires(shares.len() > 0)]
// #[requires(forall<i: Int, j: Int> 0 <= i && i < j && j < shares.len() ==> shares[i].0 != shares[j].0)]
// // This requires that every share (xi, yi) actually lies on the polynomial P
// #[requires(forall<i: Int> 0 <= i && i < shares.len() ==> shares[i].1 == eval_poly(p_coeffs, shares[i].0))]
// #[ensures(result == eval_poly(p_coeffs, 0))] // The result IS the secret (constant term)
// pub(crate) fn lagrange_interpolate_at_zero_c(shares: &[(u8, u8)], p_coeffs: Ghost<Seq<u8>>) -> u8 {
//     let mut secret = 0u8;
// 
//     for i in 0..shares.len() {
//         // ... (your existing implementation) ...
//         
//         // During the loop, we'd add an invariant that 'secret' is 
//         // building the partial sum of the Lagrange formula.
//         #[invariant(secret == partial_lagrange_sum(shares, i, 0))]
//         
//         secret = gf256_add(secret, gf256_mul(li, yi));
//     }
//     secret
// }

#[cfg(test)]
mod tests {
    use super::*;

    // Reuse GF(256) ops from same module if needed
    use crate::galois_fields::{gf256_mul, gf256_add};

    #[test]
    fn test_lagrange_interpolate_single_share() {
        let shares = vec![(5u8, 123u8)];
        assert_eq!(lagrange_interpolate_at_zero(&shares), 123);
    }

    // #[test]
    // fn test_lagrange_interpolate_two_shares_linear() {
    //     // P(x) = 42 + 3x over GF(256)
    //     // P(1) = 45, P(2) = 48
    //     let shares = vec![(1, 45), (2, 48)];
    //     let result = lagrange_interpolate_at_zero(&shares);
    //     assert_eq!(result, 42);
    // }

    #[test]
    fn test_lagrange_interpolate_two_shares_linear() {
        // Define: P(x) = 42 + 3x in GF(256)
        let p = |x: u8| gf256_add(42, gf256_mul(3, x));

        let shares = vec![(1, p(1)), (2, p(2))];

        let result = lagrange_interpolate_at_zero(&shares);
        assert_eq!(result, 42);
    }


    #[test]
    fn test_lagrange_interpolate_three_shares_quadratic() {
        // P(x) = 7x² + 5x + 200
        // Generate P(1), P(2), P(3)
        let x_vals = [1u8, 2, 3];
        let mut shares = Vec::new();
        for &x in &x_vals {
            let x2 = gf256_mul(x, x);
            let y = gf256_add(
                gf256_add(
                    gf256_mul(7, x2),
                    gf256_mul(5, x)
                ),
                200
            );
            shares.push((x, y));
        }

        let result = lagrange_interpolate_at_zero(&shares);
        assert_eq!(result, 200);
    }

    #[test]
    fn test_lagrange_interpolate_order_independence() {
        // P(x) = 0x + 55, so all shares should be (x, 55)
        let shares = vec![(1, 55), (2, 55), (3, 55)];
        let mut reversed = shares.clone();
        reversed.reverse();

        let s1 = lagrange_interpolate_at_zero(&shares);
        let s2 = lagrange_interpolate_at_zero(&reversed);

        assert_eq!(s1, 55);
        assert_eq!(s2, 55);
    }
}
