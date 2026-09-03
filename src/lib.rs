//! Finite-field arithmetic that does not branch on secrets.
//!
//! This crate is the math layer under secret-sharing and cipher crates. It
//! holds exactly three things and refuses to grow schemes of its own:
//!
//! * [`gf256`]: the field GF(2^8) with the AES reduction polynomial
//!   `x^8 + x^4 + x^3 + x + 1`. Addition is XOR, multiplication is a fixed
//!   eight-round mask-and-shift with no data-dependent branch or table
//!   lookup, inversion is `a^254`. Nothing here panics: the inverse of zero is
//!   `None`, and [`Gf256::inv_or_zero`] is there for the S-box convention.
//! * [`poly`]: polynomials over any [`Field`]. Horner evaluation, Lagrange
//!   interpolation at any point, and the special case at zero that
//!   threshold sharing needs.
//! * [`modular`] and [`secure`] (feature `bigint`, on by default): extended
//!   Euclid, modular inverse and modular polynomial evaluation over
//!   `num-bigint` integers, and big-integer wrappers that scrub their limbs
//!   when dropped.
//!
//! Without the `bigint` feature the crate is `no_std`.
//!
//! What is deliberately not here: AES, Shamir, key handling, randomness. Those
//! live in the crates that use this one.

#![cfg_attr(not(feature = "bigint"), no_std)]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod gf256;
pub mod poly;

#[cfg(feature = "bigint")]
pub mod modular;
#[cfg(feature = "bigint")]
pub mod secure;

pub use gf256::{Field, Gf256};
pub use poly::{evaluate, interpolate_at, interpolate_at_zero, lagrange_at_zero, PolyError};

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
mod readme_doctests {}
