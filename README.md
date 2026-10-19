# honest

Finite-field arithmetic that does not branch on secrets.

This is the math layer under secret-sharing and cipher crates. It holds
exactly three things and refuses to grow schemes of its own:

- **GF(2^8)** with the AES polynomial `x^8 + x^4 + x^3 + x + 1`. Addition is
  XOR, multiplication is eight fixed rounds of mask-and-shift, inversion is
  `a^254`. No lookup tables, no data-dependent branches, so no timing or
  cache channel. Nothing panics: the inverse of zero is `None`, and
  `inv_or_zero` exists for the S-box convention.
- **Polynomials over any `Field`**: Horner evaluation and Lagrange
  interpolation at a point or at zero, which is the operation a threshold
  scheme is built on.
- **Modular big-integer helpers** (feature `bigint`, on by default): extended
  Euclid, modular inverse, modular polynomial evaluation, and big-integer
  wrappers that scrub their limbs when dropped.

Without `bigint` the crate is `no_std` and has one dependency, `zeroize`.

## Example

```rust
use honest::{evaluate, interpolate_at_zero, Gf256};

// Two field facts from FIPS-197, the AES specification.
assert_eq!(Gf256(0x57) * Gf256(0x83), Gf256(0xc1));
assert_eq!(Gf256(0x53).inv(), Some(Gf256(0xca)));
assert_eq!(Gf256::ZERO.inv(), None);

// A degree-2 polynomial hides 0x2a in its constant term.
let secret = Gf256(0x2a);
let coeffs = [secret, Gf256(0x11), Gf256(0x07)];
let shares: Vec<(Gf256, Gf256)> = (1u8..=3)
    .map(|x| (Gf256(x), evaluate(&coeffs, Gf256(x))))
    .collect();

// Three points recover the constant term.
assert_eq!(interpolate_at_zero(&shares), Ok(secret));
```

## Why one field crate

A secret-sharing crate, a cipher crate and a custody system each tend to
carry their own GF(2^8), usually a log/exp table version that leaks through
the cache. One implementation, checked against the field axioms over every
operand pair and against the published AES test values, is what the crates
above this one should share.

## What is not here

AES, Shamir, key handling, randomness. Those belong to the crates that use
this one. If a routine here starts knowing what a "share" or a "key" is, it
is in the wrong crate.

## Verification

The test suite checks the field axioms exhaustively (commutativity and
inverses over every element, associativity and distributivity over a large
grid), the Fermat identity `a^255 = 1` for every non-zero element, the FIPS-197
values, Horner against naive evaluation, interpolation round trips over
random polynomials, and the modular helpers against hand-computed values.
The multiply and power routines are `const fn` with a fixed round count, so
their instruction sequence is the same for every input.

## Releasing

Bump `version` in Cargo.toml, add a dated `## <version> (YYYY-MM-DD)` entry to CHANGELOG.md, and push to
`main`. CI (.github/workflows/ci.yml) runs the tests and clippy, then publishes that version to crates.io.

## License

MIT OR Apache-2.0.
