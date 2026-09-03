//! Polynomials over a [`Field`]: evaluation and Lagrange interpolation.
//!
//! Coefficients are stored lowest degree first: `[c0, c1, c2]` is
//! `c0 + c1·x + c2·x²`. Every routine is generic over [`Field`] and written
//! without branching on coefficient values; the only branches are on the
//! number of points, which is public.

use crate::gf256::Field;
use core::fmt;

/// Why an interpolation was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolyError {
    /// No points were given.
    Empty,
    /// Two points share an x coordinate, so no polynomial passes through both.
    DuplicatePoint,
}

impl fmt::Display for PolyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "no points to interpolate"),
            Self::DuplicatePoint => write!(f, "two points share an x coordinate"),
        }
    }
}

#[cfg(feature = "bigint")]
impl std::error::Error for PolyError {}

/// `p(x)` by Horner's rule, highest degree first. An empty polynomial is zero.
pub fn evaluate<F: Field>(coeffs: &[F], x: F) -> F {
    coeffs.iter().rev().fold(F::ZERO, |acc, &c| acc.mul(x).add(c))
}

/// The value at `x` of the unique polynomial of degree below `points.len()`
/// passing through every `(xi, yi)` in `points`.
///
/// Refused when `points` is empty or two x coordinates coincide. With `k`
/// points this is `O(k²)` field operations and one inversion per point.
pub fn interpolate_at<F: Field>(points: &[(F, F)], x: F) -> Result<F, PolyError> {
    if points.is_empty() {
        return Err(PolyError::Empty);
    }
    for (i, (xi, _)) in points.iter().enumerate() {
        if points[..i].iter().any(|(xj, _)| xj == xi) {
            return Err(PolyError::DuplicatePoint);
        }
    }
    let mut acc = F::ZERO;
    for (i, &(xi, yi)) in points.iter().enumerate() {
        let mut num = F::ONE;
        let mut den = F::ONE;
        for (j, &(xj, _)) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            num = num.mul(x.sub(xj));
            den = den.mul(xi.sub(xj));
        }
        // `den` is a product of differences of distinct x's, hence non-zero.
        let basis = num.mul(den.inv().ok_or(PolyError::DuplicatePoint)?);
        acc = acc.add(basis.mul(yi));
    }
    Ok(acc)
}

/// [`interpolate_at`] zero: the constant term, which is what a threshold
/// scheme hides there.
pub fn interpolate_at_zero<F: Field>(points: &[(F, F)]) -> Result<F, PolyError> {
    interpolate_at(points, F::ZERO)
}
