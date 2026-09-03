#![allow(unused)]

use creusot_contracts::{
    ensures,
    logic,
    predicate,
    pure,
    requires
};

use num_bigint::BigUint;

use num_traits::{
    Zero,
    One
};


#[predicate]
pub fn acc(
    coeffs: &[BigUint],
    x: &BigUint,
    prime: &BigUint,
    result: BigUint,
    n: Int,
) -> bool {
    result == (
        sum(0, n, |j| (coeffs[j as usize].clone() * x.clone().pow(j as u32)) % prime)
    ) % prime
}

#[predicate]
pub fn acc2(
    coeffs: &[BigUint],
    x: &BigUint,
    prime: &BigUint,
    result: &BigUint
) -> bool {
    *result == sum(0, coeffs.len().into(), |j: Int| {
        (coeffs[j.to_usize()] * x.clone().pow(j.to_u32())) % prime.clone()
    }) % prime.clone()
}


#[logic]
pub fn sum(
    start: Int,
    end: Int,
    f: impl Fn(Int) -> BigUint,
) -> BigUint {
    let mut acc = BigUint::zero();
    let mut i = start;
    while i < end {
        acc = acc + f(i);
        i = i + 1;
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
pub fn sum<F: Fn(Int) -> BigUint>(start: Int, end: Int, f: F) -> BigUint {
    if start >= end { BigUint::zero() }
    else { f(start) + sum(start + 1, end, f) }
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
    while i < coeffs.len() as Int {
        acc = (acc + term(&coeffs[i as usize], x, i, prime)) % prime;
        i = i + 1;
    }
    acc
}

#[logic]
fn sum<F: Fn(Int) -> BigUint>(start: Int, end: Int, f: F) -> BigUint {
    if start >= end { BigUint::zero() }
    else { f(start) + sum(start + 1, end, f) }
}


#[predicate]
fn acc(coeffs: &[BigUint], x: &BigUint, prime: &BigUint, result: &BigUint) -> bool {
    *result == sum(0, coeffs.len().into(), |j: Int| {
        (coeffs[j.to_usize()] * x.clone().pow(j.to_u32())) % prime.clone()
    }) % prime.clone()
}

