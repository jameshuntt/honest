use crate::galois_fields::{gf256_add, gf256_div, gf256_mul, gf256_sub};

pub fn lagrange_interpolate_at_zero(shares: &[(u8, u8)]) -> u8 {
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
