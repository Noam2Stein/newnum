use crate::*;

/// Used by the [`num`] macro to convert integer literals to `Self`.
pub trait FromIntLiteral: Sized {
    /// The minimum integer literal that can be converted to `Self` without loosing precision.
    ///
    /// For an approximate minumum use `Self::MIN_APPROX_LITERAL`.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MIN_LITERAL: i128;
    /// The maximum integer literal that can be converted to `Self` without loosing precision.
    ///
    /// For an approximate maximum use `Self::MAX_APPROX_LITERAL`.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MAX_LITERAL: i128;
    /// The minimum integer literal that can be approximately converted to `Self`.
    ///
    /// This is set to `Self::MIN_LITERAL` on default.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MIN_APPROX_LITERAL: i128 = Self::MIN_LITERAL;

    /// The maximum integer literal that can be approximately converted to `Self`.
    ///
    /// This is set to `Self::MAX_LITERAL` on default.
    ///
    /// An example for the difference between precise and approximate,
    /// is `f32` which can represent numbers up to `3.4028235 × 10³⁸`,
    /// but looses integer precision after `16_777_216`.
    const MAX_APPROX_LITERAL: i128 = Self::MAX_LITERAL;

    /// Used by the [`num`] macro to convert integer literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_LITERAL` and `Self::MAX_LITERAL`.
    /// The fn is still expected to not cause undefined behavior if the value is out of range,
    /// because the fn is only unsafe to prevent it from being called manually.
    unsafe fn from_int_literal(value: i128) -> Self;

    /// Used by the [`num_approx`] macro to convert integer literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_APPROX_LITERAL` and `Self::MAX_APPROX_LITERAL`.
    /// The fn is still expected to not cause undefined behavior if the value is out of range,
    /// because the fn is only unsafe to prevent it from being called manually.
    unsafe fn approx_from_int_literal(value: i128) -> Self;
}

/// Used by the [`num`] macro to convert float literals to `Self`.
pub trait FromFloatLiteral: FromIntLiteral {
    /// Used by the [`num`] macro to convert float literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_LITERAL` and `Self::MAX_LITERAL`.
    unsafe fn from_float_literal(value: f64) -> Self;

    /// Used by the [`num_approx`] macro to convert float literals to `Self`.
    ///
    /// Is unsafe because the caller must ensure that the value is between `Self::MIN_APPROX_LITERAL` and `Self::MAX_APPROX_LITERAL`.
    unsafe fn approx_from_float_literal(value: f64) -> Self;

    fn pi() -> Self {
        internal_num!(3.141592653589793238462643383279502884197169399375105820974944592307816406286208998628034825342117067982148086513282306647093844609550582231725359408128481117450284102701938521105559644622948954930381964428810975665933446128475648233786783165271201909145648566923460348610454326648213393607260249141273724587006606315588174881520920962829254091715364367892590360011330530548820466521384146951941511609433057270365759591953092186117381932611793105118548074462379962749567351885752724891227938183011949128831426076896280457)
    }
}

macro_rules! int_impl {
    ($ty:ident) => {
        impl FromIntLiteral for $ty {
            const MIN_LITERAL: i128 = 0;
            const MAX_LITERAL: i128 = $ty::MAX as i128;

            unsafe fn from_int_literal(value: i128) -> Self {
                value as Self
            }

            unsafe fn approx_from_int_literal(value: i128) -> Self {
                value as Self
            }
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

impl FromIntLiteral for u128 {
    const MIN_LITERAL: i128 = 0;
    const MAX_LITERAL: i128 = i128::MAX;

    unsafe fn from_int_literal(value: i128) -> Self {
        value as Self
    }

    unsafe fn approx_from_int_literal(value: i128) -> Self {
        value as Self
    }
}

impl FromIntLiteral for f32 {
    const MIN_LITERAL: i128 = 0;
    const MAX_LITERAL: i128 = 16_777_216; // 2^24
    const MIN_APPROX_LITERAL: i128 = 0;
    const MAX_APPROX_LITERAL: i128 = Self::MAX as i128;

    unsafe fn from_int_literal(value: i128) -> Self {
        value as Self
    }

    unsafe fn approx_from_int_literal(value: i128) -> Self {
        value as Self
    }
}

impl FromIntLiteral for f64 {
    const MIN_LITERAL: i128 = 0;
    const MAX_LITERAL: i128 = 9_007_199_254_740_992; // 2^53
    const MIN_APPROX_LITERAL: i128 = 0;
    const MAX_APPROX_LITERAL: i128 = i128::MAX; // f64 can represent all u128 values

    unsafe fn from_int_literal(value: i128) -> Self {
        value as Self
    }

    unsafe fn approx_from_int_literal(value: i128) -> Self {
        value as Self
    }
}

impl FromFloatLiteral for f32 {
    unsafe fn from_float_literal(value: f64) -> Self {
        value as Self
    }

    unsafe fn approx_from_float_literal(value: f64) -> Self {
        value as Self
    }
}

impl FromFloatLiteral for f64 {
    unsafe fn from_float_literal(value: f64) -> Self {
        value
    }

    unsafe fn approx_from_float_literal(value: f64) -> Self {
        value
    }
}
