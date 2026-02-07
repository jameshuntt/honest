use num_bigint::BigUint;
use num_traits::{Zero, One};
/// Evaluates the polynomial at a given `x` value modulo `prime`.
pub fn evaluate_polynomial(
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
use crate::types::secure_types::SecureBigUint;
pub fn secure_evaluate_polynomial(
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
