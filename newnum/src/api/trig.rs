/// Trigonometric API (`sin`, `cos`, `tan`).
/// Functions take in `self` and return `Output`.
///
/// * `sin(a) = opposite / hypotenuse`,
/// * `cos(a) = adjacent / hypotenuse`,
/// * `tan(a) = sin(a) / cos(a) = opposite / adjacent`,
pub trait Trig {
    /// The output of trigonometric functions.
    /// Represents the triangle side ratio.
    ///
    /// For abstract numbers `Output` is expected to be `Self`.
    /// For non abstract numbers, `Output` can be whatever makes sense.
    ///
    /// For example `Angle::Output = Ratio`.
    type Output;

    /// Computes the sine (in radians for abstract types).
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn sin(self) -> Self::Output;

    /// Computes the cosine (in radians for abstract types).
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn cos(self) -> Self::Output;

    /// Computes the tangent (in radians for abstract types).
    /// `tan(a) = sin(a) / cos(a)`
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn tan(self) -> Self::Output;
}

/// Inverse trigonometric API (`asin`, `acos`, `atan`).
/// Functions take in `self` and return `Output`.
///
/// `sin(asin(x)) = x`, `tan(atan(x)) = x`...
pub trait ATrig {
    /// The output of inverse trigonometric functions.
    /// Represents the angle.
    ///
    /// For abstract numbers `Output` is expected to be `Self`.
    /// For non abstract numbers, `Output` can be whatever makes sense.
    ///
    /// For example `Ratio::Output = Angle`.
    type Output;

    /// Computes the arcsine (in radians for abstract types).
    /// Return value is in the range [-pi/2, pi/2].
    ///
    /// May panic or return NaN if the number is outside the range [-1, 1].
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn asin(self) -> Self::Output;

    /// Computes the arccosine (in radians for abstract types).
    /// Return value is in the range [0, pi].
    ///
    /// May panic or return NaN if the number is outside the range [-1, 1].
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn acos(self) -> Self::Output;

    /// Computes the arctangent (in radians for abstract types).
    /// Return value is in the range [-pi/2, pi/2].
    ///
    /// The precision of this function might be non-deterministic based on the type.
    fn atan(self) -> Self::Output;
}

pub trait Hyper {
    type Output;

    fn sinh(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
}

pub trait AHyper {
    type Output;

    fn asinh(self) -> Self::Output;
    fn acosh(self) -> Self::Output;
    fn atanh(self) -> Self::Output;
}

macro_rules! impl_for_primitive {
    ($type:ty) => {
        impl Trig for $type {
            type Output = Self;

            #[inline(always)]
            fn sin(self) -> Self::Output {
                self.sin()
            }
            #[inline(always)]
            fn cos(self) -> Self::Output {
                self.cos()
            }
            #[inline(always)]
            fn tan(self) -> Self::Output {
                self.tan()
            }
        }

        impl ATrig for $type {
            type Output = Self;

            #[inline(always)]
            fn asin(self) -> Self::Output {
                self.asin()
            }
            #[inline(always)]
            fn acos(self) -> Self::Output {
                self.acos()
            }
            #[inline(always)]
            fn atan(self) -> Self::Output {
                self.atan()
            }
        }

        impl Hyper for $type {
            type Output = Self;

            #[inline(always)]
            fn sinh(self) -> Self::Output {
                self.sinh()
            }
            #[inline(always)]
            fn cosh(self) -> Self::Output {
                self.cosh()
            }
            #[inline(always)]
            fn tanh(self) -> Self::Output {
                self.tanh()
            }
        }

        impl AHyper for $type {
            type Output = Self;

            #[inline(always)]
            fn asinh(self) -> Self::Output {
                self.asinh()
            }
            #[inline(always)]
            fn acosh(self) -> Self::Output {
                self.acosh()
            }
            #[inline(always)]
            fn atanh(self) -> Self::Output {
                self.atanh()
            }
        }
    };
}
impl_for_primitive!(f32);
impl_for_primitive!(f64);
