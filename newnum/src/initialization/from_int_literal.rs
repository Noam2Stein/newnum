/// Used by the [`num`] macro to convert integer literals to `Self`.
pub trait FromIntLiteral: Sized {
    /// The minimum integer literal that can be converted to `Self` without loosing precision.
    ///
    /// For an approximate minumum use `Self::MIN_APPROX_LITERAL`.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MIN_LITERAL: u128;
    /// The maximum integer literal that can be converted to `Self` without loosing precision.
    ///
    /// For an approximate maximum use `Self::MAX_APPROX_LITERAL`.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MAX_LITERAL: u128;
    /// The minimum integer literal that can be approximately converted to `Self`.
    ///
    /// This is set to `Self::MIN_LITERAL` on default.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MIN_APPROX_LITERAL: u128 = Self::MIN_LITERAL;

    /// The maximum integer literal that can be approximately converted to `Self`.
    ///
    /// This is set to `Self::MAX_LITERAL` on default.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MAX_APPROX_LITERAL: u128 = Self::MAX_LITERAL;

    /// Used by the [`num`] macro to convert integer literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_LITERAL` and `Self::MAX_LITERAL`.
    /// The fn is still expected to not cause undefined behavior if the value is out of range,
    /// because the fn is only unsafe to prevent it from being called manually.
    unsafe fn from_int_literal(value: u128) -> Self;

    /// Used by the [`num_approx`] macro to convert integer literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_APPROX_LITERAL` and `Self::MAX_APPROX_LITERAL`.
    /// The fn is still expected to not cause undefined behavior if the value is out of range,
    /// because the fn is only unsafe to prevent it from being called manually.
    unsafe fn approx_from_int_literal(value: u128) -> Self;
}

macro_rules! int_impl {
    ($ty:ident) => {
        impl FromIntLiteral for $ty {
            const MIN_LITERAL: u128 = 0;
            const MAX_LITERAL: u128 = $ty::MAX as u128;

            unsafe fn from_int_literal(value: u128) -> Self {
                value as Self
            }

            unsafe fn approx_from_int_literal(value: u128) -> Self {
                value as Self
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

impl FromIntLiteral for f32 {
    const MIN_LITERAL: u128 = 0;
    const MAX_LITERAL: u128 = 16_777_216; // 2^24
    const MIN_APPROX_LITERAL: u128 = 0;
    const MAX_APPROX_LITERAL: u128 = Self::MAX as u128;

    unsafe fn from_int_literal(value: u128) -> Self {
        value as Self
    }

    unsafe fn approx_from_int_literal(value: u128) -> Self {
        value as Self
    }
}

impl FromIntLiteral for f64 {
    const MIN_LITERAL: u128 = 0;
    const MAX_LITERAL: u128 = 9_007_199_254_740_992; // 2^53
    const MIN_APPROX_LITERAL: u128 = 0;
    const MAX_APPROX_LITERAL: u128 = u128::MAX; // f64 can represent all u128 values

    unsafe fn from_int_literal(value: u128) -> Self {
        value as Self
    }

    unsafe fn approx_from_int_literal(value: u128) -> Self {
        value as Self
    }
}
