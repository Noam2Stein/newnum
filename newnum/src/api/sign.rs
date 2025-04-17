/// Trait for types that have a mathamatical sign (number / number-container).
///
/// doesn't require the type to represent positive / negative / zero values.
///
/// doesn't have functions like `abs`,
/// because for example if the type can't represent positive values but only negative ones,
/// it can't return a positive value out of ```abs```.
pub trait Signed {
    /// The bool equivalent of `Self`.
    ///
    /// For a number this is always `bool`,
    /// but for a number-container this is the type of the container that holds `bool` values.
    /// For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    type BoolMapped;

    /// Returns `true` if `self` is positive in a `3-value-sign`.
    ///
    /// A `3-value-sign` can be either positive, negative or zero.
    /// This means that for `self = 0`, `false` is returned.
    ///
    /// * For a `2-value-sign` use `is_bin_positive`.
    ///
    /// * For number-container types, this function maps each element of the container to a `bool`.
    ///   For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    fn is_positive(&self) -> Self::BoolMapped;

    /// Returns `true` if `self` is negative in a `3-value-sign`.
    ///
    /// A `3-value-sign` can be either positive, negative or zero.
    /// This means that for `self = 0`, `false` is returned.
    ///
    /// * For a `2-value-sign` use `is_bin_negative`.
    ///
    /// * For number-container types, this function maps each element of the container to a `bool`.
    ///   For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    fn is_negative(&self) -> Self::BoolMapped;

    /// Returns `true` if `self` is zero.
    ///
    /// * For number-container types, this function maps each element of the container to a `bool`.
    ///   For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    fn is_zero(&self) -> Self::BoolMapped;

    /// Returns `true` if `self` is positive in a `2-value-sign` / `binary-sign`.
    ///
    /// A `2-value-sign` can be either positive or negative, and cannot be zero.
    /// This means that for `self = +0`, `true` is returned, and for `self = -0`, `false` is returned.
    /// For types that cannot distinguish between `+0` and `-0`, `0` is treated as positive.
    ///
    /// * For a `3-value-sign` use `is_positive`.
    ///
    /// * For number-container types, this function maps each element of the container to a `bool`.
    ///   For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    fn is_bin_positive(&self) -> Self::BoolMapped;

    /// Returns `true` if `self` is negative in a `2-value-sign` / `binary-sign`.
    ///
    /// A `2-value-sign` can be either positive or negative, and cannot be zero.
    /// This means that for `self = +0`, `false` is returned, and for `self = -0`, `true` is returned.
    /// For types that cannot distinguish between `+0` and `-0`, `0` is treated as positive.
    ///
    /// * For a `3-value-sign` use `is_negative`.
    ///
    /// * For number-container types, this function maps each element of the container to a `bool`.
    ///   For example, `Vec2<T>` will be mapped to `Vec2<bool>`.
    fn is_bin_negative(&self) -> Self::BoolMapped;
}

//
//
//
// POSITIVE, NEGATIVE, ZERO
//
//
//

/// Trait for types that can represent positive values (number / number-container).
///
/// Doesn't mean the type is always positive,
/// For that use `OnlyPositive`.
pub trait Positive: Signed {
    fn abs(self) -> Self;
}

/// Trait for types that can represent negative values (number / number-container).
///
/// Doesn't mean the type is always negative,
/// For that use `OnlyNegative`.
pub trait Negative: Signed {
    fn neg_abs(self) -> Self;
}
/// Trait for types that can represent zero (number / number-container).
///
/// Doesn't mean the type is always zero,
/// For that use `OnlyZero`.
pub trait Zero: Signed {
    fn zero() -> Self;
}

//
//
//
// NEGATIVE TRAITS
//
//
//

/// Trait for types that are never positive (number / number-container).
///
/// * In the future when rust supports negative traits (`!SomeTrait`),
/// this trait will be replaced with `!Positive`.
pub trait NotPositive: Signed {}

/// Trait for types that are never negative (number / number-container).
///
/// * In the future when rust supports negative traits (`!SomeTrait`),
/// this trait will be replaced with `!Negative`.
pub trait NotNegative: Signed {}

/// Trait for types that are never zero (number / number-container).
///
/// * In the future when rust supports negative traits (`!SomeTrait`),
/// this trait will be replaced with `!Zero`.
pub trait NotZero: Signed {}

//
//
//
// TRAIT ALIASES
//
//
//

/// Trait for types that are always positive (number / number-container).
///
/// * This trait is a shortcut for `Positive + NotNegative + NotZero`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait AlwaysPositive: Positive + NotNegative + NotZero {}

/// Trait for types that are always negative (number / number-container).
///
/// * This trait is a shortcut for `Negative + NotPositive + NotZero`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait AlwaysNegative: Negative + NotPositive + NotZero {}

/// Trait for types that are always zero (number / number-container).
///
/// * This trait is a shortcut for `Zero + NotPositive + NotNegative`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait AlwaysZero: Zero + NotPositive + NotNegative {}

/// Trait for types that are always either positive or zero (number / number-container).
///
/// * This trait is a shortcut for `Positive + Zero + NotNegative`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait PositiveOrZero: NotNegative + Zero + Positive {}

/// Trait for types that are always either negative or zero (number / number-container).
///
/// * This trait is a shortcut for `Negative + Zero + NotPositive`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait NegativeOrZero: Negative + Zero + NotPositive {}

/// Trait for types that are always either positive or negative, so never zero (number / number-container).
///
/// * This trait is a shortcut for `Positive + Negative + NotZero`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait PositiveOrNegative: Negative + NotZero + Positive {}

/// Trait for types that can represent both positive values, negative values and zero (number / number-container).
///
/// * This trait is a shortcut for `Positive + Negative + Zero`, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases),
///   this will be made into a trait alias.
pub trait FullySigned: Negative + Zero + Positive {}

impl<T: Positive + NotNegative + NotZero> AlwaysPositive for T {}
impl<T: Negative + NotPositive + NotZero> AlwaysNegative for T {}
impl<T: Zero + NotPositive + NotNegative> AlwaysZero for T {}
impl<T: Positive + Zero + NotNegative> PositiveOrZero for T {}
impl<T: Negative + Zero + NotPositive> NegativeOrZero for T {}
impl<T: Positive + Negative + NotZero> PositiveOrNegative for T {}
impl<T: Positive + Negative + Zero> FullySigned for T {}

//
//
//
// SIGNUM
//
//
//

/// Trait for `signum` functions.
///
/// This logically requires:
/// - that if `Self` can be positive, it can represent `1`,
/// - that if `Self` can be negative, it can represent `-1`.
///
/// If this requirement wasn't met, the `signum` function would be logically impossible to implement.
pub trait Signum: Signed {
    /// Returns either `1`, `-1` or `0` based on the number's `3-value-sign`:
    /// - `self > 0 => 1`,
    /// - `self < 0 => -1`,
    /// - `self == 0 => 0`.
    ///
    /// A `3-value-sign` can be either positive, negative or zero,
    /// In comparison to a `2-value-sign` / `binary-sign` which can be either positive or negative, and cannot be zero.
    ///
    /// * For a `2-value-sign` use `bin_signum`.
    ///
    /// * This function is named `signumt` and not `signum` because in the standard-library,
    /// `f32/f64::signum` acts using a `2-value-sign` and not a `3-value-sign`.
    fn signumt(self) -> Self;

    /// Returns either `1` or `-1` based on the number's `2-value-sign`:
    /// - `self > 0 => 1`,
    /// - `self < 0 => -1`,
    /// - `self == +0 => 1`,
    /// - `self == -0 => -1`.
    ///
    /// A `2-value-sign` can be either positive or negative, and cannot be zero.
    /// This means that for `self = +0`, `1` is returned, and for `self = -0`, `-1` is returned.
    /// For types that cannot distinguish between `+0` and `-0`, `0` is treated as positive.
    ///
    /// * For a `3-value-sign` use `signumt`.
    fn bin_signum(self) -> Self;
}

//
//
//
// IMPLEMENTATIONS
//
//
//

macro_rules! uint_impl {
    ($type:ident) => {
        impl Signed for $type {
            type BoolMapped = bool;

            fn is_positive(&self) -> Self::BoolMapped {
                *self != 0
            }
            fn is_negative(&self) -> Self::BoolMapped {
                false
            }

            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0
            }

            fn is_bin_positive(&self) -> Self::BoolMapped {
                true
            }
            fn is_bin_negative(&self) -> Self::BoolMapped {
                false
            }
        }

        impl Positive for $type {
            fn abs(self) -> Self {
                self
            }
        }
        impl Zero for $type {
            fn zero() -> Self {
                0
            }
        }
        impl NotNegative for $type {}

        impl Signum for $type {
            fn signumt(self) -> Self {
                (self != 0) as $type
            }
            fn bin_signum(self) -> Self {
                1
            }
        }
    };
}
uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(u128);
uint_impl!(usize);

macro_rules! sint_impl {
    ($type:ident) => {
        impl Signed for $type {
            type BoolMapped = bool;

            fn is_positive(&self) -> Self::BoolMapped {
                *self > 0
            }
            fn is_negative(&self) -> Self::BoolMapped {
                *self < 0
            }

            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0
            }

            fn is_bin_positive(&self) -> Self::BoolMapped {
                *self >= 0
            }
            fn is_bin_negative(&self) -> Self::BoolMapped {
                *self < 0
            }
        }

        impl Positive for $type {
            fn abs(self) -> Self {
                self.abs()
            }
        }
        impl Negative for $type {
            fn neg_abs(self) -> Self {
                -self.abs()
            }
        }
        impl Zero for $type {
            fn zero() -> Self {
                0
            }
        }

        impl Signum for $type {
            fn signumt(self) -> Self {
                self.signum()
            }
            fn bin_signum(self) -> Self {
                if self >= 0 {
                    1
                } else {
                    -1
                }
            }
        }
    };
}
sint_impl!(i8);
sint_impl!(i16);
sint_impl!(i32);
sint_impl!(i64);
sint_impl!(i128);
sint_impl!(isize);

macro_rules! float_impl {
    ($type:ident) => {
        impl Signed for $type {
            type BoolMapped = bool;

            fn is_positive(&self) -> Self::BoolMapped {
                self.is_sign_positive() && *self != 0.0
            }
            fn is_negative(&self) -> Self::BoolMapped {
                self.is_sign_negative() && *self != 0.0
            }

            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0.0
            }

            fn is_bin_positive(&self) -> Self::BoolMapped {
                $type::is_sign_positive(*self)
            }
            fn is_bin_negative(&self) -> Self::BoolMapped {
                $type::is_sign_negative(*self)
            }
        }

        impl Positive for $type {
            fn abs(self) -> Self {
                self.abs()
            }
        }
        impl Negative for $type {
            fn neg_abs(self) -> Self {
                -self.abs()
            }
        }
        impl Zero for $type {
            fn zero() -> Self {
                0.0
            }
        }

        impl Signum for $type {
            fn signumt(self) -> Self {
                if self == 0.0 {
                    0.0
                } else {
                    self.signum()
                }
            }
            fn bin_signum(self) -> Self {
                $type::signum(self)
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
