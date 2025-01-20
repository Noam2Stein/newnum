use splat_attribs::splat_attribs;

use crate::{FloatingEquivalent, Int, Positive, WholeEquivalent};

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

macro_rules! int_cast_impl {
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

        impl IRoot for $type {
            #[inline(always)]
            fn isqrt(self) -> Self {
                self.trunc_sqrt()
            }

            #[inline(always)]
            fn icbrt(self) -> Self {
                self.trunc_cbrt()
            }
        }
    };
}
int_cast_impl!(u8);
int_cast_impl!(u16);
int_cast_impl!(u32);
int_cast_impl!(i8);
int_cast_impl!(i16);
int_cast_impl!(i32);
splat_attribs! {
    #[cfg(target_pointer_width = "32")]:

    int_cast_impl!(usize);
    int_cast_impl!(isize);
}

macro_rules! int_checked_cast_impl {
    ($type:ident) => {
        impl TruncRoot for $type {
            #[inline(always)]
            fn trunc_sqrt(self) -> Self {
                if self.abs() < 2 ^ 52 {
                    (self as f64).trunc_sqrt() as _
                } else {
                    int_sqrt(self)
                }
            }

            #[inline(always)]
            fn trunc_cbrt(self) -> Self {
                if self.abs() < 2 ^ 52 {
                    (self as f64).trunc_cbrt() as _
                } else {
                    int_cbrt(self)
                }
            }
        }

        impl IRoot for $type {
            #[inline(always)]
            fn isqrt(self) -> Self {
                self.trunc_sqrt()
            }

            #[inline(always)]
            fn icbrt(self) -> Self {
                self.trunc_cbrt()
            }
        }
    };
}
int_checked_cast_impl!(u64);
int_checked_cast_impl!(u128);
int_checked_cast_impl!(i64);
int_checked_cast_impl!(i128);
splat_attribs! {
    #[cfg(target_pointer_width = "64")]:

    int_checked_cast_impl!(usize);
    int_checked_cast_impl!(isize);
}

fn int_sqrt<T: Int>(value: T) -> T {
    if value.is_negative() {
        panic!("attempt to get sqrt of negative value")
    } else if value.is_zero() {
        value
    } else {
        let mut low = T::u7(1);
        let mut high = value;

        while low <= high {
            let mid = (low + high) / T::u7(2);
            let square = mid * mid;

            if square == value {
                return mid;
            } else if square < value {
                low = mid + T::u7(1);
            } else {
                high = mid - T::u7(1);
            }
        }

        high
    }
}

fn int_cbrt<T: Int>(value: T) -> T {
    if value.is_negative() {
        panic!("attempt to get cbrt of negative value")
    } else if value.is_zero() {
        value
    } else {
        let mut low = T::u7(1);
        let mut high = value;

        while low <= high {
            let mid = (low + high) / T::u7(2);
            let square = mid * mid * mid;

            if square == value {
                return mid;
            } else if square < value {
                low = mid + T::u7(1);
            } else {
                high = mid - T::u7(1);
            }
        }

        high
    }
}
