/// Trigonometric API (`sin`, `cos`, `tan`, `cot`, `sec`, `csc`).
/// Functions take in `self` and return `Output`.
///
/// * `sin(a) = opposite / hypotenuse`,
/// * `cos(a) = adjacent / hypotenuse`,
/// * `tan(a) = sin(a) / cos(a) = opposite / adjacent`,
/// * `cot(a) = 1 / tan(a) = cos(a) / sin(a) = adjacent / opposite`,
/// * `sec(a) = 1 / cos(a) = hypotenuse / adjacent`,
/// * `csc(a) = 1 / sin(a) = hypotenuse / opposite`.
pub trait Trig {
    /// The output of trigonometric functions where `Self` is the angle and `Output` is the triangle ratio.
    ///
    /// For `Num` types (defined as abstract numbers) `Output` is expected to be `Self`.
    /// For non abstract numbers, `Output` can be whatever makes sense.
    /// For example `Angle::Output = Ratio`.
    type Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `opposite / hypotenuse`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn sin(self) -> Self::Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `adjacent / hypotenuse`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn cos(self) -> Self::Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `opposite / adjacent`.
    /// `tan(a) = sin(a) / cos(a)`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn tan(self) -> Self::Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `adjacent / opposite`.
    /// `cot(a) = 1 / tan(a) = cos(a) / sin(a)`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn cot(self) -> Self::Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `hypotenuse / adjacent`.
    /// `sec(a) = 1 / cos(a)`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn sec(self) -> Self::Output;

    /// Trigonometric function that uses `self` as an angle to determine the right triangle ratio `hypotenuse / opposite`.
    /// `csc(a) = 1 / sin(a)`.
    ///
    /// for `Num` types (defined as abstract numbers) `self` is used as radians and not degress.
    /// For non `Num` types the measurement unit needs to be defined in the type or initialization, for example `Angle::degress(60)`.
    fn csc(self) -> Self::Output;
}

/// Inverse trigonometric API (`asin`, `acos`, `atan`, `acot`, `asec`, `acsc`).
/// Functions take in `self` and return `Output`.
///
/// `sin(asin(x)) = x`
pub trait ATrig {
    /// For `Num` types (defined as abstract numbers) `Output` is expected to be `Self`.
    /// For non numbers, `Output` can be whatever makes sense.
    /// Keep in mind this good to follow rule `<Self::Output as Trig>::Output = Self`.
    /// For example `Ratio::Output = Angle`.
    type Output;

    fn asin(self) -> Self::Output;
    fn acos(self) -> Self::Output;
    fn atan(self) -> Self::Output;
    fn acot(self) -> Self::Output;
}

/// Hyperbolic API (`sinh`, `cosh`, `tanh`, `cot`)
pub trait Hyper {
    type Output;

    fn sinh(self) -> Self::Output;
    fn cosh(self) -> Self::Output;
    fn tanh(self) -> Self::Output;
    fn coth(self) -> Self::Output;
}

pub trait AHyper {
    type Output;

    fn asinh(self) -> Self::Output;
    fn acosh(self) -> Self::Output;
    fn atanh(self) -> Self::Output;
    fn acoth(self) -> Self::Output;
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
            #[inline(always)]
            fn cot(self) -> Self::Output {
                1.0 / self.tan()
            }
            #[inline(always)]
            fn sec(self) -> Self::Output {
                1.0 / self.cos()
            }
            #[inline(always)]
            fn csc(self) -> Self::Output {
                1.0 / self.sin()
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
            #[inline(always)]
            fn acot(self) -> Self::Output {
                (1.0 / self).atan()
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
            #[inline(always)]
            fn coth(self) -> Self::Output {
                1.0 / self.tanh()
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
            #[inline(always)]
            fn acoth(self) -> Self::Output {
                (1.0 / self).atanh()
            }
        }
    };
}
impl_for_primitive!(f32);
impl_for_primitive!(f64);
