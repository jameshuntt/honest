#![allow(unused)]




use creusot_std::{logic::Int, macros::{ensures, logic}, model::View};
use num_bigint::BigUint;

use num_traits::{
    Zero,
    One
};

use creusot_std::prelude::*;

fn i32_to_int(x: i32) -> Int {
    *Int::new(x.into())
}

#[logic]
fn add(x: Int, y: Int) -> Int { x + y }
#[logic]
#[ensures(add(x, 0) == x)]
fn add_zero(x: Int) {
    // proof: trivial in this case
}
#[logic]
fn int_to_i32(x: Int) -> i32 {
    x as i32
}
// #[predicate]
pub fn acc(
    coeffs: &[BigUint],
    x: &BigUint,
    prime: &BigUint,
    result: BigUint,
    n: Int,
) -> bool {
    result == (
        sum(i32_to_int(0), n, |j|
            (
                coeffs[j].clone() * x.clone().pow(j as u32)
            ) % prime)
    ) % prime
}

#[predicate]
pub fn acc2(
    coeffs: &[BigUint],
    x: &BigUint,
    prime: &BigUint,
    result: &BigUint
) -> bool {
    *result == sum(0.view(), coeffs.len().into(), |j: Int| {
        (coeffs[j.to_usize()] * x.clone().pow(j.to_u32())) % prime.clone()
    }) % prime.clone()
}

// #[predicate]
// fn acc_3(
//     coeffs: &[BigUint],
//     x: &BigUint,
//     prime: &BigUint,
//     result: &BigUint
// ) -> bool {
//     *result == sum(0.view(), coeffs.len().into(), |j: Int| {
//         (coeffs[j.to_usize()] * x.clone().pow(j.to_u32())) % prime.clone()
//     }) % prime.clone()
// }
// 







// #[logic]
pub fn sum(
    start: Int,
    end: Int,
    f: impl Fn(Int) -> BigUint,
) -> BigUint {
    let mut acc = BigUint::zero();
    let mut i = start;
    while i < end {
        acc = acc + f(i);
        i = i + 1.view();
    }
    acc
}
#[logic]
pub fn sum<F: Fn(Int) -> BigUint>(start: Int, end: Int, f: F) -> BigUint {
    if start >= end { BigUint::zero() }
    else { f(start) + sum(start + 1.view(), end, f) }
}

#[ensures(result@ == sum(start, end, f))]
pub fn sum_impl(
    start: Int,
    end: Int,
    f: impl Fn(Int) -> BigUint,
) -> BigUint {
    let mut acc = BigUint::zero();
    let mut i = start;
    while i < end {
        acc = acc + f(i);
        i = i + 1.view();
    }
    acc
}









#[logic]
pub fn pow(x: &BigUint, exp: Int) -> BigUint {
    let mut result = BigUint::one();
    let mut i = 0;
    while i < exp {
        result = result * x;
        i = i + 1;
    }
    result
}









#[logic]
pub fn term(
    coeff: &BigUint,
    x: &BigUint,
    i: Int,
    prime: &BigUint
) -> BigUint {
    (coeff * pow(x, i)) % prime
}






#[logic]
pub fn poly(
    coeffs: &[BigUint],
    x: &BigUint,
    prime: &BigUint
) -> BigUint {
    let mut acc = BigUint::zero();
    let mut i = 0;
    while i < coeffs.len().view() {
        acc = (acc + term(&coeffs[i as usize], x, i.view(), prime)) % prime;
        i = i + 1;
    }
    acc
}



