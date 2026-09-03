// src/gf256.rs


pub mod add;
pub(crate) use add::{gf256_add_runtime as gf256_add};
// #[logic(open)]
#[ensures(result == a ^ b)]
pub(crate) fn gf256_add_og(a: u8, b: u8) -> u8 {
    a ^ b
}
// 1. Write the function exactly as normal Rust code
pub fn gf256_add_2(a: u8, b: u8) -> u8 {
    a ^ b
}

// // 2. Attach the logic "externally" in the same file
// #[extern_spec(self)]
// impl _ {
//     // #[logic] 
//     // #[open] // This allows the prover to see the 'a ^ b' inside
//     fn gf256_add_2(a: u8, b: u8) -> u8;
// }

#[ensures(result == gf256_add(a, b))] // This now works!
pub fn call_add(a: u8, b: u8) -> u8 {
    gf256_add_2(a, b)
}
// #[logic]
#[ensures(result == a ^ b)] // same as add
pub(crate) fn gf256_sub(a: u8, b: u8) -> u8 {
    a ^ b // same as add in GF(2^8)
}

// #[logic]
pub(crate) fn gf256_mul(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0u8;
    while b != 0 {
        if b & 1 != 0 {
            result ^= a;
        }
        let carry = a & 0x80;
        a <<= 1;
        if carry != 0 {
            a ^= 0x1b; // irreducible polynomial x^8 + x^4 + x^3 + x + 1
        }
        b >>= 1;
    }
    result
}

// #[logic]
#[requires(a != 0)]
#[ensures(gf256_mul(result, a) == 1)]
pub(crate) fn gf256_inv(a: u8) -> u8 {
    assert!(a != 0, "cannot invert 0 in GF(256)");
    // Fermat’s little theorem: a^(2^8 - 2) = a^-1 in GF(2^8)
    gf256_exp(a, 254)
}

// #[logic]
#[requires(b != 0)]
#[ensures(gf256_mul(result, b) == a)]
pub(crate) fn gf256_div(a: u8, b: u8) -> u8 {
    gf256_mul(a, gf256_inv(b))
}

// #[logic]
#[ensures(old(exp) == 0 ==> result == 1)] // if exp == 0, result == 1
#[ensures(old(exp) == 1 ==> result == base)] // if exp == 1, result == base
pub(crate) fn gf256_exp(mut base: u8, mut exp: u8) -> u8 {
    let mut result = 1u8;
    while exp != 0 {
        if exp & 1 != 0 {
            result = gf256_mul(result, base);
        }
        base = gf256_mul(base, base);
        exp >>= 1;
    }
    result
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::galois_fields::*;

    #[test]
    fn test_gf256_add_sub() {
        for a in 0..=255 {
            for b in 0..=255 {
                assert_eq!(gf256_add(a, b), a ^ b);
                assert_eq!(gf256_sub(a, b), a ^ b);
                assert_eq!(gf256_add(a, b), gf256_sub(a, b)); // symmetry in GF(2^8)
            }
        }
    }

    #[test]
    fn test_gf256_mul_basic() {
        assert_eq!(gf256_mul(0, 0), 0);
        assert_eq!(gf256_mul(0, 1), 0);
        assert_eq!(gf256_mul(1, 0), 0);
        assert_eq!(gf256_mul(1, 1), 1);
        assert_eq!(gf256_mul(1, 255), 255);
        assert_eq!(gf256_mul(255, 1), 255);
    }

    #[test]
    fn test_gf256_mul_distributive() {
        // a*(b+c) = ab + ac
        for a in 1..=10 {
            for b in 1..=10 {
                for c in 1..=10 {
                    let left = gf256_mul(a, gf256_add(b, c));
                    let right = gf256_add(gf256_mul(a, b), gf256_mul(a, c));
                    assert_eq!(left, right, "Distributive failed for a={}, b={}, c={}", a, b, c);
                }
            }
        }
    }

    #[test]
    fn test_gf256_inv_and_div() {
        for a in 1..=255 {
            let inv = gf256_inv(a);
            assert_eq!(gf256_mul(a, inv), 1, "a * inv(a) != 1 for a = {}", a);

            for b in 1..=255 {
                let div = gf256_div(a, b);
                assert_eq!(gf256_mul(div, b), a, "a/b * b != a for a = {}, b = {}", a, b);
            }
        }
    }

    #[test]
    #[should_panic(expected = "cannot invert 0 in GF(256)")]
    fn test_gf256_inv_zero_should_panic() {
        gf256_inv(0);
    }

    #[test]
    fn test_gf256_exp_properties() {
        for base in 1..=10 {
            assert_eq!(gf256_exp(base, 0), 1);
            assert_eq!(gf256_exp(base, 1), base);
            assert_eq!(gf256_exp(base, 2), gf256_mul(base, base));
        }
    }

    #[test]
    fn test_gf256_exp_fermat_identity() {
        for a in 1..=255 {
            assert_eq!(gf256_exp(a, 254), gf256_inv(a)); // a^(2^8-2) == a^-1
        }
    }
}
// src/gf256.rs (Continued)

// --- Logical Contracts for Field Axioms ---

// Proves that multiplication distributes over addition: a * (b + c) = (a * b) + (a * c)
// #[pure]
#[ensures(result == true)]
pub(crate) fn is_mul_distributive(a: u8, b: u8, c: u8) -> bool {
    gf256_mul(a, gf256_add(b, c)) == gf256_add(gf256_mul(a, b), gf256_mul(a, c))
}

// Proves that addition is associative: a + (b + c) = (a + b) + c
// NOTE: Since addition is simple XOR, this is easy for Z3 to prove (XOR is naturally associative).
// #[pure]
#[ensures(result == true)]
pub(crate) fn is_add_associative(a: u8, b: u8, c: u8) -> bool {
    gf256_add(a, gf256_add(b, c)) == gf256_add(gf256_add(a, b), c)
}

// Proves that multiplication is associative: a * (b * c) = (a * b) * c
// NOTE: This is complex due to the reduction step in gf256_mul.
// #[pure]
#[ensures(result == true)]
pub(crate) fn is_mul_associative(a: u8, b: u8, c: u8) -> bool {
    gf256_mul(a, gf256_mul(b, c)) == gf256_mul(gf256_mul(a, b), c)
}

// Proves that multiplication is commutative: a * b = b * a
// #[pure]
#[ensures(result == true)]
pub(crate) fn is_mul_commutative(a: u8, b: u8) -> bool {
    gf256_mul(a, b) == gf256_mul(b, a)
}

// #[pure]
pub(crate) fn gf256_mul_ct(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0u8;
    // We loop exactly 8 times (deterministic timing)
    for _ in 0..8 {
        // Mask: if the LSB of b is 1, mask is 0xFF, else 0x00
        let mask = !((b & 1).wrapping_sub(1));
        result ^= a & mask;

        // Reduction step: if MSB of a is 1, we XOR with the polynomial
        let carry = (a >> 7) & 1;
        let poly_mask = !((carry).wrapping_sub(1));
        a = (a << 1) ^ (poly_mask & 0x1b);

        b >>= 1;
    }
    result
}

// // In a real crate, these would be pre-calculated constants
// lazy_static::lazy_static! {
//     static ref LOG: [u8; 256] = generate_log_table();
//     static ref EXP: [u8; 512] = generate_exp_table(); // 512 to avoid modulo 255
// }

// #[pure]
// pub fn gf256_mul_fast(a: u8, b: u8) -> u8 {
//     if a == 0 || b == 0 { return 0; }
//     let sum = (LOG[a as usize] as usize) + (LOG[b as usize] as usize);
//     EXP[sum] // EXP is doubled to handle the overflow without % 255
// }

pub trait GaloisField: Sized + Copy {
    fn add(self, other: Self) -> Self;
    fn sub(self, other: Self) -> Self;
    fn mul(self, other: Self) -> Self;
    fn inv(self) -> Self;
    
    const ZERO: Self;
    const ONE: Self;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GF256(pub u8);

impl GaloisField for GF256 {
    fn add(self, other: Self) -> Self { GF256(self.0 ^ other.0) }
    fn sub(self, other: Self) -> Self { self.add(other) }
    fn mul(self, other: Self) -> Self { GF256(gf256_mul_ct(self.0, other.0)) }
    fn inv(self) -> Self { 
        // Use your Fermat implementation here
        unimplemented!() 
    }
    const ZERO: Self = GF256(0);
    const ONE: Self = GF256(1);
}

#[ensures(result == gf256_mul_reference(a, b))]
pub fn gf256_mul_optimized(a: u8, b: u8) -> u8 {
    // Calling the table-based version
    gf256_mul_fast(a, b)
}

pub fn generate_tables() -> ([u8; 256], [u8; 512]) {
    let mut log = [0u8; 256];
    let mut exp = [0u8; 512];
    
    let mut x = 1u8;
    for i in 0..255 {
        exp[i] = x;
        exp[i + 255] = x; // Duplicate for overflow handling
        log[x as usize] = i as u8;
        
        // Multiply by 2 (the generator)
        let carry = x & 0x80;
        x <<= 1;
        if carry != 0 {
            x ^= 0x1b; // The irreducible polynomial
        }
    }
    
    // Note: log[0] is technically undefined (-infinity)
    // We usually set it to 0 or 255 depending on the impl.
    (log, exp)
}

pub struct GF256Tables {
    pub log: [u8; 256],
    pub exp: [u8; 512],
}

pub const TABLES: GF256Tables = {
    let mut log = [0u8; 256];
    let mut exp = [0u8; 512];
    let mut x = 1u8;
    let mut i = 0;
    
    while i < 255 {
        exp[i] = x;
        exp[i + 255] = x;
        log[x as usize] = i as u8;
        
        let carry = x & 0x80;
        x = (x << 1) ^ (if carry != 0 { 0x1b } else { 0 });
        i += 1;
    }
    
    GF256Tables { log, exp }
};

// #[pure]
#[requires(a != 0 && b != 0)] // Logs aren't defined for 0
pub fn gf256_mul_fast(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 { return 0; }
    
    let l_a = TABLES.log[a as usize];
    let l_b = TABLES.log[b as usize];
    
    // Adding logs is equivalent to multiplying elements
    // We use the 512-length exp table to avoid a % 255
    TABLES.exp[(l_a as usize) + (l_b as usize)]
}


use creusot_std::macros::{ensures, logic, requires};

#[logic]
// #[pure]
pub fn mul_spec(a: u8, b: u8) -> u8 {
    // This is a logical representation of the field multiplication
    // Creusot will use this to reason about what 'multiplication' means.
    ghost_mul(a, b)
}

// A ghost function is only for verification and isn't compiled into the binary
#[logic]
fn ghost_mul(mut a: u8, mut b: u8) -> u8 {
    // ... insert your original bit-looping logic here ...
    // The prover uses this as the "definition" of GF multiplication
}


// #[predicate]
// pub fn is_valid_log_table(log: [u8; 256], exp: [u8; 512]) -> bool {
//     // For all x from 1 to 255, exp[log[x]] == x
//     forall<x: u8> 1 <= x && x <= 255 ==> 
//         exp[log[x as usize] as usize] == x
// }
// 
// #[requires(a != 0 && b != 0)]
// #[ensures(result == mul_spec(a, b))] // This links implementation to math
// pub fn gf256_mul_fast(a: u8, b: u8) -> u8 {
//     let l_a = TABLES.log[a as usize];
//     let l_b = TABLES.log[b as usize];
//     
//     // We tell the prover to trust that the sum of logs 
//     // maps correctly to the exponent table
//     let res = TABLES.exp[(l_a as usize) + (l_b as usize)];
//     res
// }


#[logic]
fn pow2_logic(i: Int) -> u8 {
    // This defines 2^i in the field for the prover
    if i == 0.view() { 1 }
    else { ghost_mul(2, pow2_logic(i - 1.view())) }
}


// pub const TABLES: GF256Tables = {
//     let mut log = [0u8; 256];
//     let mut exp = [0u8; 512];
//     let mut x = 1u8;
//     let mut i = 0;
//     
//     while i < 255 {
//         // --- LOOP INVARIANTS FOR CREUSOT ---
//         #[invariant(0 <= i && i <= 255)]
//         #[invariant(x == pow2_logic(i))] // x is always the i-th power of 2
//         #[invariant(forall<j: Int> 0 <= j && j < i ==> exp[j] == pow2_logic(j))]
//         #[invariant(forall<j: Int> 0 <= j && j < i ==> log[exp[j] as usize] == j as u8)]
//         
//         exp[i] = x;
//         exp[i + 255] = x;
//         log[x as usize] = i as u8;
//         
//         let carry = x & 0x80;
//         x = (x << 1) ^ (if carry != 0 { 0x1b } else { 0 });
//         i += 1;
//     }
//     
//     GF256Tables { log, exp }
// };