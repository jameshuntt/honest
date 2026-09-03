
use creusot_std::{logic::Seq, macros::{ensures, logic, requires}, model::View};
use num_bigint::BigUint;
use num_traits::{Zero, One};

use crate::{
    galois_fields::{gf256_add, gf256_mul},
    // pure_functions::*
};


/// Evaluates a polynomial modulo prime.
#[requires(!prime.is_zero())]
#[ensures(
    forall<i: Int>
    0 <= i && i < coefficients.len()
    ==> acc(coefficients, x, prime, result, i)
)]
// #[logic]
pub fn evaluate_polynomial_unclear(
    coefficients: &[BigUint],
    x: &BigUint,
    prime: &BigUint
) -> BigUint {
    let mut result = BigUint::zero();
    let mut power_of_x = BigUint::one();

    for coeff in coefficients {
        result = (result + coeff * &power_of_x) % prime;
        power_of_x = (power_of_x * x) % prime;
    }

    result
}


/// Evaluates the polynomial at a given `x` value modulo `prime`.
#[requires(!prime.is_zero())]
// #[logic]
pub(crate) fn evaluate_polynomial(
    coefficients: &[BigUint],
    x: &BigUint,
    prime: &BigUint
) -> BigUint {
    let mut result = BigUint::zero();
    let mut power_of_x = BigUint::one();

    for coeff in coefficients {
        let term = (coeff * &power_of_x) % prime;
        result = (result + term) % prime;
        power_of_x = (power_of_x * x) % prime;
    }

    result
}

use secrecy::ExposeSecret;
use crate::types::secure_types::SecureBigUint;

#[requires(!prime.is_zero())]
// #[logic]
pub(crate) fn secure_evaluate_polynomial(
    coefficients: &[SecureBigUint],
    x: &BigUint,
    prime: &BigUint
) -> BigUint {
    let mut result = BigUint::zero();
    let mut power_of_x = BigUint::one();

    for coeff in coefficients {
        let term = (&coeff.0 * &power_of_x) % prime;
        result = (result + term) % prime;
        power_of_x = (&power_of_x * x) % prime;
    }

    result
}

// use crate::helpers::gf256::*;

// Represents a polynomial where coeffs[i] is the coefficient of x^i
#[logic]
fn eval_poly(coeffs: Seq<u8>, x: u8) -> u8 {
    if coeffs.len() == 0.view() {
        0
    } else {
        // Horner's Method: coeffs[0] + x * eval(remaining_coeffs, x)
        gf256_add(
            coeffs[0], 
            gf256_mul(x, eval_poly(coeffs.tail(), x))
        )
    }
}

#[logic]
fn basis_poly(shares: Seq<(u8, u8)>, i: Int, x: u8) -> u8 {
    // This represents the Product of (x - xj) / (xi - xj)
    // We define this so the prover knows what the "ideal" result is
    basis_product(shares, i, 0, x)
}

#[logic]
fn basis_product(shares: Seq<(u8, u8)>, i: Int, current: Int, x: u8) -> u8 {
    if current >= shares.len() {
        1
    } else if i == current {
        basis_product(shares, i, current + 1, x)
    } else {
        let num = gf256_sub(x, shares[current].0);
        let den = gf256_sub(shares[i].0, shares[current].0);
        gf256_mul(gf256_div(num, den), basis_product(shares, i, current + 1, x))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_poly_eval() {
        let coeffs = vec![
            1u8.to_biguint().unwrap(),
            2u8.to_biguint().unwrap(),
            3u8.to_biguint().unwrap()
        ]; // 1 + 2x + 3x^2
        let x = 2u8.to_biguint().unwrap(); // x = 2
        let prime = 97u8.to_biguint().unwrap(); // mod 97
        assert_eq!(evaluate_polynomial(&coeffs, &x, &prime), (1u8 + 2*2 + 3*4).into()); // 1 + 4 + 12 = 17
    }

    #[test]
    fn prime_module_order() {
        let coeffs = vec![
            1u8.to_biguint().unwrap(),
            2u8.to_biguint().unwrap(),
            3u8.to_biguint().unwrap()
        ]; // 1 + 2x + 3x^2
        
        let sec_coeffs = vec![
            SecureBigUint(1u8.to_biguint().unwrap()),
            SecureBigUint(2u8.to_biguint().unwrap()),
            SecureBigUint(3u8.to_biguint().unwrap())
        ]; // 1 + 2x + 3x^2

        let x = 2u8.to_biguint().unwrap(); // x = 2
        let prime = 97u8.to_biguint().unwrap(); // mod 97


        let a = secure_evaluate_polynomial(&sec_coeffs, &x, &prime);
        let b = evaluate_polynomial(&coeffs, &x, &prime);

        assert_eq!(a, b)
    }
}


// 
// // Evaluate the polynomial at a given x
// fn evaluate_polynomial(coefficients: &Vec<BigUint>, x: &BigUint, prime: &BigUint) -> BigUint {
//     let mut result = BigUint::zero();
//     let mut power_of_x = BigUint::one();
// 
//     for coeff in coefficients {
//         let term = (coeff * &power_of_x) % prime;
//         result = (result + term) % prime;
//         power_of_x = (power_of_x * x) % prime;
//     }
// 
//     result
// }
// 
