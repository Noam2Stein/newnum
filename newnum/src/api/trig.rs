pub trait Trig {
    type Output;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn tan(self) -> Self::Output;
    fn cot(self) -> Self::Output;
}

pub trait ATrig {
    type Output;

    fn asin(self) -> Self::Output;
    fn acos(self) -> Self::Output;
    fn atan(self) -> Self::Output;
    fn acot(self) -> Self::Output;
}

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
