use crate::*;

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
