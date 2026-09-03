/// Runtime GF(256) addition (XOR)
pub(crate) fn gf256_add_runtime(a: u8, b: u8) -> u8 {
    a ^ b
}

use creusot_std::prelude::*;

#[inline(always)]
fn gf256_add_logic(a: Int, b: Int) -> Int {
    // a ^ b
        (a + b) % *Int::new(256)

}
#[inline(always)]
fn gf256_add_logic_b(a: Int, b: Int) -> Int {
    let modulo = *Int::new(256);
    (a + b) % modulo
}
#[logic]
#[ensures(
    forall(a: Int, b: Int) ==>
        gf256_add_logic(a,b) == gf256_add_logic(b,a)
)]
fn gf256_add_commutative(a: Int, b: Int) {
    // XOR is commutative: a ^ b == b ^ a
}
#[test]
pub(crate) fn test_gf256_add() {
    
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let r = gf256_add_runtime(a,b);

            let a_int: Int = *Int::new(a.into()); 
            let b_int: Int = *Int::new(b.into());

            let r_logic = gf256_add_logic(a_int, b_int);
            let r_expected = (a_int + (b_int)) % *Int::new(256); // normal arithmetic mod 256
            // let r_expected_int = *Int::new(r_expected);

            assert!(*Int::new(r.into()) as Int == r_logic);
            assert!(*Int::new(r.into()) as Int == r_expected);
            assert!(r_logic == r_expected);
        }
    }
}
#[test]
pub(crate) fn test_gf256_add_2() {
    for a in 0u8..=255 {
        for b in 0u8..=255 {
            let r = gf256_add_runtime(a,b);
            let r_v = *Int::new(r.into());

            let a_int: Int = Int::new(a.into()).into_inner(); 
            let b_int: Int = Int::new(b.into()).into_inner();

            let r_l = (a_int + (b_int)) % Int::new(256).into_inner();

            assert!(r_v == r_l);
        }
    }
}

// pub(crate) fn test_gf256_add_runtime() {
//     for a in 0u8..=255 {
//         for b in 0u8..=255 {
//             let r = gf256_add_runtime(a,b);
//             let expected = a.wrapping_add(b) % 256;
//             assert!(r == expected); // ✅ pure runtime check
//         }
//     }
// }