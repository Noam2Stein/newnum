use crate::WholeEquivalent;

/// Trait for the round API (`round`, `floor`...) which may be used for numbers, number containers, or anything that makes sense.
///
/// Functions in this trait return `Self` so this isn't round-to-int,
/// for that use [`IRound`].
///
/// Can also and should also be implemented for types that only represent round numbers (int types).
/// This is because if you have a `value: impl Num`,
/// it makes sense to round it even if it may be an int meaning already round.
pub trait Round: Sized {
    /// Rounds to the closer whole number.
    ///
    /// For example: `1.2 => 1.0`, `1.8 => 2.0`, `-1.3 => -1.0`.
    fn round(self) -> Self;

    /// Rounds down.
    ///
    /// For example: `1.2 => 1.0`, `1.8 => 1.0`, `-1.3 => -2.0`.
    fn floor(self) -> Self;

    /// Rounds up.
    ///
    /// For example: `1.2 => 2.0`, `1.8 => 2.0`, `-1.3 => -1.0`.
    fn ceil(self) -> Self;

    /// If positive rounds down, else rounds up.
    ///
    /// Effectively removes the fractional part of the number by rounding towards zero.
    /// For example: `1.2 => 1.0`, `1.8 => 1.0`, `-1.3 => -1.0`.
    fn trunc(self) -> Self;

    /// If positive rounds up, else rounds down.
    ///
    /// Effectively rounds away from zero.
    /// For example: `1.2 => 2.0`, `1.8 => 2.0`, `-1.3 => -2.0`.
    fn atrunc(self) -> Self;
}

/// Trait for a round-to-int API (iround, ifloor...).
///
/// ```Round``` returns ```Self```,
/// where ```IRound``` returns ```Self::Whole``` which represents the whole equivalent of ```Self```.
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
pub trait IRound: Sized + WholeEquivalent {
    fn iround(self) -> Self::Whole;
    fn ifloor(self) -> Self::Whole;
    fn iceil(self) -> Self::Whole;
    fn itrunc(self) -> Self::Whole;
    fn iatrunc(self) -> Self::Whole;
}

macro_rules! int_impl {
    ($type:ident) => {
        impl Round for $type {
            #[inline(always)]
            fn round(self) -> Self {
                self
            }

            #[inline(always)]
            fn floor(self) -> Self {
                self
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                self
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                self
            }

            #[inline(always)]
            fn atrunc(self) -> Self {
                self
            }
        }

        impl IRound for $type {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self
            }

            #[inline(always)]
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
    ($type:ident) => {
        impl Round for $type {
            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }
            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }
            #[inline(always)]
            fn trunc(self) -> Self {
                self.trunc()
            }
            #[inline(always)]
            fn atrunc(self) -> Self {
                if self.is_sign_positive() {
                    self.ceil()
                } else {
                    self.floor()
                }
            }
        }

        impl IRound for $type {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self.round() as _
            }
            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self.floor() as _
            }
            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self.ceil() as _
            }
            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self.trunc() as _
            }
            #[inline(always)]
            fn iatrunc(self) -> Self::Whole {
                self.atrunc() as _
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
