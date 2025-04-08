use crate::*;

/// Trait for numbers or number containers that are always round (for example ```i32```).
pub trait Whole: WholeEquivalent<Whole = Self> {}

impl<T: WholeEquivalent<Whole = Self>> Whole for T {}

/// Trait for numbers/number-containers that have whole/integer equivilants.
/// The `Whole` type should be the lightest whole type that can represent the entire range of `Self`.
pub trait WholeEquivalent: Round {
    type Whole: Whole;

    /// Rounds to the closer whole number, and converts into an integer.
    ///
    /// For example: `1.2 => 1`, `1.8 => 2`, `-1.3 => -1`.
    fn iround(self) -> Self::Whole;

    /// Rounds down, and converts into an integer.
    ///
    /// For example: `1.2 => 1`, `1.8 => 1`, `-1.3 => -2`.
    fn ifloor(self) -> Self::Whole;

    /// Rounds up, and converts into an integer.
    ///
    /// For example: `1.2 => 2`, `1.8 => 2`, `-1.3 => -1`.
    fn iceil(self) -> Self::Whole;

    /// Rounds towards zero (if positive rounds down, else rounds up), and converts into an integer.
    ///
    /// Effectively removes the fractional.
    /// For example: `1.2 => 1`, `1.8 => 1`, `-1.3 => -1`.
    fn itrunc(self) -> Self::Whole;

    /// Rounds away from zero (if positive rounds up, else rounds down), and converts into an integer.
    ///
    /// For example: `1.2 => 2`, `1.8 => 2`, `-1.3 => -2`.
    fn iatrunc(self) -> Self::Whole;

    fn isqrt(self) -> Self::Whole
    where
        Self: TruncRoot,
    {
        self.trunc_sqrt().iround()
    }

    fn icbrt(self) -> Self::Whole
    where
        Self: TruncRoot,
    {
        self.trunc_cbrt().iround()
    }
}

macro_rules! int_impl {
    ($type:ident) => {
        impl WholeEquivalent for $type {
            type Whole = Self;

            fn iround(self) -> Self::Whole {
                self
            }
            fn ifloor(self) -> Self::Whole {
                self
            }
            fn iceil(self) -> Self::Whole {
                self
            }
            fn itrunc(self) -> Self::Whole {
                self
            }
            fn iatrunc(self) -> Self::Whole {
                self
            }
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

macro_rules! float_impl {
    ($type:ident => $whole:ident) => {
        impl WholeEquivalent for $type {
            type Whole = $whole;

            fn iround(self) -> Self::Whole {
                self.round() as $whole
            }
            fn ifloor(self) -> Self::Whole {
                self.floor() as $whole
            }
            fn iceil(self) -> Self::Whole {
                self.ceil() as $whole
            }
            fn itrunc(self) -> Self::Whole {
                self.trunc() as $whole
            }
            fn iatrunc(self) -> Self::Whole {
                self.atrunc() as $whole
            }
        }
    };
}
float_impl!(f32 => i32);
float_impl!(f64 => i64);
