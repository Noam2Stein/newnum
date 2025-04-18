use std::ops::{Mul, Neg};

/// Represents a `3-value-sign`.
///
/// A `3-value-sign` can be either positive, negative or zero,
/// while a `2-value-sign` / `bit-sign` can only positive or negative.
///
/// In a `3-value-sign`, both `+0` and `-0` are considered zero,
/// while in a `2-value-sign`, `+0` is considered positive and `-0` is considered negative.
///
/// * For a `2-value-sign` equivalent, use [`BitSign`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sign {
    Negative,
    Zero,
    Positive,
}

/// Represents a `2-value-sign`.
///
/// A `2-value-sign` / `bit-sign` can be either positive or negative,
/// while a `3-value-sign` can be positive, negative or zero.
///
/// In a `2-value-sign`, `+0` is considered positive and `-0` is considered negative,
/// while in a `3-value-sign`, both `+0` and `-0` are considered zero.
///
/// * For a `3-value-sign` equivalent, use [`Sign`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BitSign {
    Negative,
    Positive,
}

//
//
//
// IMPL FOR SIGN
//
//
//

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Negative => Self::Positive,
            Self::Zero => Self::Zero,
            Self::Positive => Self::Negative,
        }
    }
}

impl Mul for Sign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Zero, _) => Self::Zero,
            (_, Self::Zero) => Self::Zero,
            (Self::Positive, rhs) => rhs,
            (lhs, Self::Positive) => lhs,
            (Self::Negative, Self::Negative) => Self::Positive,
        }
    }
}

//
//
//
// IMPL FOR BIT-SIGN
//
//
//

impl Neg for BitSign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Negative => Self::Positive,
            Self::Positive => Self::Negative,
        }
    }
}

impl Mul for BitSign {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Positive, rhs) => rhs,
            (lhs, Self::Positive) => lhs,
            (Self::Negative, Self::Negative) => Self::Positive,
        }
    }
}
