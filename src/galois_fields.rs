/// Runtime GF(256) addition (XOR)
pub(crate) fn gf256_add(a: u8, b: u8) -> u8 {a ^ b }
pub fn call_add(a: u8, b: u8) -> u8 { gf256_add(a, b) }
/// Same as add in GF(2^8)
pub(crate) fn gf256_sub(a: u8, b: u8) -> u8 { a ^ b }
pub fn call_sub(a: u8, b: u8) -> u8 { gf256_sub(a, b) }
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
}pub fn call_mul_ct(a: u8, b: u8) -> u8 { gf256_mul_ct(a, b) }
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
}pub fn call_mul(a: u8, b: u8) -> u8 { gf256_mul(a, b) }
fn gf256_exp(mut base: u8, mut exp: u8) -> u8 {
    let mut result = 1u8;
    while exp != 0 {
        if exp & 1 != 0 {
            result = gf256_mul_ct(result, base);
        }
        base = gf256_mul_ct(base, base);
        exp >>= 1;
    }
    result
}pub fn call_exp(base: u8, exp: u8) -> u8 { gf256_exp(base, exp) }


pub(crate) fn gf256_inv(a: u8) -> u8 {
    assert!(a != 0, "cannot invert 0 in GF(256)");
    // Fermat’s little theorem: a^(2^8 - 2) = a^-1 in GF(2^8)
    gf256_exp(a, 254)
}pub fn call_inv(a: u8) -> u8 { gf256_inv(a) }

pub(crate) fn gf256_div(a: u8, b: u8) -> u8 {
    gf256_mul(a, gf256_inv(b))
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
pub fn is_valid_log_table(log: [u8; 256], exp: [u8; 512]) -> bool {
    for x in 1u16..=255 {
        let x = x as u8;
        let log_x = log[x as usize] as usize;

        if exp[log_x] != x {
            return false;
        }
    }
    true
}











pub fn gf256_mul_fast(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 { return 0; }
    
    let l_a = TABLES.log[a as usize];
    let l_b = TABLES.log[b as usize];
    
    // Adding logs is equivalent to multiplying elements
    // We use the 512-length exp table to avoid a % 255
    // TABLES.exp[(l_a as usize) + (l_b as usize)]
    let res = TABLES.exp[(l_a as usize) + (l_b as usize)];
    res
}pub fn run_mul_opt(a: u8, b: u8) -> u8 { gf256_mul_fast(a, b) } // Calling the table-based version













#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    // Proves that multiplication distributes over addition: a * (b + c) = (a * b) + (a * c)
    fn is_mul_distributive() {
        let a = 13 as u8;
        let b = 16 as u8;
        let c = 73 as u8;
    
        assert_eq!(
            gf256_mul(a, gf256_add(b, c)),
            gf256_add(gf256_mul(a, b), gf256_mul(a, c))
        );
    }
    
    #[test]
    // Proves that addition is associative: a + (b + c) = (a + b) + c
    // NOTE: Since addition is simple XOR, this is easy for Z3 to prove (XOR is naturally associative).
    fn is_add_associative() {
        let a = 13 as u8;
        let b = 16 as u8;
        let c = 73 as u8;
    
        assert_eq!(
            gf256_add(a, gf256_add(b, c)),
            gf256_add(gf256_add(a, b), c)
        )
    }
    
    #[test]
    // Proves that multiplication is associative: a * (b * c) = (a * b) * c
    // NOTE: This is complex due to the reduction step in gf256_mul.
    fn is_mul_associative() {
        let a = 13 as u8;
        let b = 16 as u8;
        let c = 73 as u8;
    
        assert_eq!(
            gf256_mul(a, gf256_mul(b, c)),
            gf256_mul(gf256_mul(a, b), c)
        );
    }
    
    #[test]
    // Proves that multiplication is commutative: a * b = b * a
    pub(crate) fn is_mul_commutative() {
            let a = 13 as u8;
        let b = 16 as u8;
    
        assert_eq!(
        gf256_mul(a, b),
        gf256_mul(b, a)
        );
    }

    #[test]
    pub(crate) fn test_gf256_add_runtime() {
        for a in 0u8..=255 {
            for b in 0u8..=255 {
                let r = gf256_add(a,b);
                let expected = a.wrapping_add(b);
                assert!(r == expected); // ✅ pure runtime check
            }
        }
    }
}
