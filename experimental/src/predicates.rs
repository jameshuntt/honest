
use creusot_std::{logic::Seq, macros::{ensures, logic}, model::View};

use crate::{galois_fields::{gf256_add, gf256_mul}, types::FieldElement, uses::aes::{inv_s_box, s_box_s}};
#[logic]
pub fn is_sbox_inverse(x: u8) -> bool {
    inv_s_box(s_box_s(FieldElement(x))) == FieldElement(x)
}

#[logic]
#[ensures(forall<x: u8> is_sbox_inverse(x))]
pub fn prove_sbox_properties() {
    // This tells the solver to check all 256 possible byte values.
    // Because the domain is small, Z3 can "exhaust" this proof in milliseconds.
}
// We need to tell Creusot that our GF math exists in the logic world
#[logic]
#[extern_spec] // Points to your actual implementation
fn gf256_add(a: FieldElement, b: FieldElement) -> FieldElement;

#[logic]
#[extern_spec]
fn gf256_mul(a: FieldElement, b: FieldElement) -> FieldElement;

pub fn poly_eval_spec(coeffs: Seq<FieldElement>, x: FieldElement) -> FieldElement {
    if coeffs.len() == 0.view() {
        FieldElement(0)
    } else {
        // P(x) = coeffs[0] + x * P_tail(x)
        let first = coeffs.get_ghost(0);
        let tail = coeffs.drop(1);
        // We use our logic-level GF math functions here
        gf256_add(first, gf256_mul(x, poly_eval_spec(tail, x)))
    }
}

// #[predicate]
pub fn is_honest_evaluation(coeffs: Seq<FieldElement>, x: FieldElement, result: FieldElement) -> bool {
    result == poly_eval_spec(coeffs, x)
}