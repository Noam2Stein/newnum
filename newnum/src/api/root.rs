use crate::{FloatingEquivalent, Whole, WholeEquivalent};

pub trait Root {
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
}
pub trait TruncRoot {
    fn trunc_sqrt(self) -> Self;
    fn trunc_cbrt(self) -> Self;
}
pub trait IRoot: WholeEquivalent {
    fn isqrt(self) -> Self;
    fn icbrt(self) -> Self;
}

impl<T: Whole + TruncRoot> IRoot for T {
    #[inline(always)]
    fn isqrt(self) -> Self {
        self.trunc_sqrt()
    }
    #[inline(always)]
    fn icbrt(self) -> Self {
        self.trunc_sqrt()
    }
}

macro_rules! float_impl {
    ($type:ident) => {
        impl Root for $type {
            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            #[inline(always)]
            fn cbrt(self) -> Self {
                self.cbrt()
            }
        }

        impl TruncRoot for $type {
            #[inline(always)]
            fn trunc_sqrt(self) -> Self {
                self.sqrt().trunc()
            }
            #[inline(always)]
            fn trunc_cbrt(self) -> Self {
                self.cbrt().trunc()
            }
        }

        impl IRoot for $type {
            #[inline(always)]
            fn isqrt(self) -> Self {
                self.trunc_sqrt() as _
            }
            #[inline(always)]
            fn icbrt(self) -> Self {
                self.trunc_cbrt() as _
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);

macro_rules! int_impl {
    ($type:ident) => {
        impl TruncRoot for $type {
            #[inline(always)]
            fn trunc_sqrt(self) -> Self {
                (self as <Self as FloatingEquivalent>::Floating).trunc_sqrt() as _
            }

            #[inline(always)]
            fn trunc_cbrt(self) -> Self {
                (self as <Self as FloatingEquivalent>::Floating).trunc_cbrt() as _
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
