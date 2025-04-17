use crate::*;

/// Trait for the round API (`round`, `floor`...)
/// which may be used for numbers, number containers, or anything that makes sense.
///
/// Functions in this trait return `Self` so this isn't round-to-int,
/// for that use [`IRound`].
///
/// Can also and should also be implemented for types that only represent round numbers (int types).
/// This is because if you have a `value: impl Num`,
/// it makes sense to round it even if it may be an int meaning already round.
pub trait Round: Sized + Zero {
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

    /// Rounds towards zero. If positive rounds down, else rounds up.
    ///
    /// Effectively removes the fractional.
    /// For example: `1.2 => 1.0`, `1.8 => 1.0`, `-1.3 => -1.0`.
    fn trunc(self) -> Self;

    /// Rounds away from zero. If positive rounds up, else rounds down.
    ///
    /// For example: `1.2 => 2.0`, `1.8 => 2.0`, `-1.3 => -2.0`.
    fn atrunc(self) -> Self;

    /// Returns the fractional part of self.
    fn fract(self) -> Self;
}

macro_rules! int_impl {
    ($type:ident) => {
        impl Round for $type {
            #[inline(always)]
            fn round(self) -> Self {
                self
            }
            fn floor(self) -> Self {
                self
            }
            fn ceil(self) -> Self {
                self
            }
            fn trunc(self) -> Self {
                self
            }
            fn atrunc(self) -> Self {
                self
            }
            fn fract(self) -> Self {
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
            fn round(self) -> Self {
                self.round()
            }
            fn floor(self) -> Self {
                self.floor()
            }
            fn ceil(self) -> Self {
                self.ceil()
            }
            fn trunc(self) -> Self {
                self.trunc()
            }
            fn atrunc(self) -> Self {
                if self.is_sign_positive() {
                    self.ceil()
                } else {
                    self.floor()
                }
            }
            fn fract(self) -> Self {
                self.fract()
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
