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
    };
}
impl_for_primitive!(f32);
impl_for_primitive!(f64);
