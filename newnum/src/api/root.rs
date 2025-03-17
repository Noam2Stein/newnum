use newnum_proc_macros::crate_int;
use splat_attribs::splat_attribs;

use crate::{FloatingEquivalent, Int, Num, Positive, WholeEquivalent};

mod ext {
    use super::*;

    pub trait SqrtExt: Root<2> {
        fn sqrt(self) -> Self::Root;
    }
    pub trait CbrtExt: Root<3> {
        fn cbrt(self) -> Self::Root;
    }
    pub trait TruncSqrtExt: TruncRoot<2> {
        fn trunc_sqrt(self) -> Self::Root;
    }
    pub trait TruncCbrtExt: TruncRoot<3> {
        fn trunc_cbrt(self) -> Self::Root;
    }
    pub trait ISqrtExt: IRoot<2> {
        fn isqrt(self) -> <Self::Root as WholeEquivalent>::Whole;
    }
    pub trait ICbrtExt: IRoot<3> {
        fn icbrt(self) -> <Self::Root as WholeEquivalent>::Whole;
    }

    impl<T: Root<2>> SqrtExt for T {
        fn sqrt(self) -> Self::Root {
            self.root()
        }
    }
    impl<T: Root<3>> CbrtExt for T {
        fn cbrt(self) -> Self::Root {
            self.root()
        }
    }
    impl<T: TruncRoot<2>> TruncSqrtExt for T {
        fn trunc_sqrt(self) -> Self::Root {
            self.trunc_root()
        }
    }
    impl<T: TruncRoot<3>> TruncCbrtExt for T {
        fn trunc_cbrt(self) -> Self::Root {
            self.trunc_root()
        }
    }
    impl<T: IRoot<2>> ISqrtExt for T {
        fn isqrt(self) -> <Self::Root as WholeEquivalent>::Whole {
            self.iroot()
        }
    }
    impl<T: IRoot<3>> ICbrtExt for T {
        fn icbrt(self) -> <Self::Root as WholeEquivalent>::Whole {
            self.iroot()
        }
    }
}
pub use ext::*;

pub trait RootEquivalent<const P: usize> {
    type Root;
}

pub trait Root<const P: usize>: RootEquivalent<P> {
    fn root(self) -> Self::Root;
}
pub trait TruncRoot<const P: usize>: RootEquivalent<P> {
    fn trunc_root(self) -> Self::Root;
}
pub trait IRoot<const P: usize>: RootEquivalent<P, Root: WholeEquivalent> {
    fn iroot(self) -> <Self::Root as WholeEquivalent>::Whole;
}

impl<T: Num, const P: usize> RootEquivalent<P> for T {
    type Root = T;
}

macro_rules! float_impl {
    ($type:ident) => {
        impl Root<2> for $type {
            #[inline(always)]
            fn root(self) -> Self::Root {
                self.sqrt()
            }
        }
        impl Root<3> for $type {
            #[inline(always)]
            fn root(self) -> Self::Root {
                self.cbrt()
            }
        }

        impl TruncRoot<2> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                self.sqrt().trunc()
            }
        }
        impl TruncRoot<3> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                self.cbrt().trunc()
            }
        }

        impl IRoot<2> for $type {
            #[inline(always)]
            fn iroot(self) -> <Self::Root as WholeEquivalent>::Whole {
                TruncRoot::<2>::trunc_root(self) as _
            }
        }
        impl IRoot<3> for $type {
            #[inline(always)]
            fn iroot(self) -> <Self::Root as WholeEquivalent>::Whole {
                TruncRoot::<3>::trunc_root(self) as _
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);

macro_rules! int_cast_impl {
    ($type:ident) => {
        impl TruncRoot<2> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                TruncRoot::<2>::trunc_root(self as <Self as FloatingEquivalent>::Floating) as _
            }
        }
        impl TruncRoot<3> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                TruncRoot::<3>::trunc_root(self as <Self as FloatingEquivalent>::Floating) as _
            }
        }

        impl IRoot<2> for $type {
            #[inline(always)]
            fn iroot(self) -> Self {
                TruncRoot::<2>::trunc_root(self)
            }
        }
        impl IRoot<3> for $type {
            #[inline(always)]
            fn iroot(self) -> Self {
                TruncRoot::<3>::trunc_root(self)
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
        impl TruncRoot<2> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                if self.abs() < 2 ^ 52 {
                    TruncRoot::<2>::trunc_root(self as f64) as _
                } else {
                    int_sqrt(self)
                }
            }
        }
        impl TruncRoot<3> for $type {
            #[inline(always)]
            fn trunc_root(self) -> Self {
                if self.abs() < 2 ^ 52 {
                    TruncRoot::<3>::trunc_root(self as f64) as _
                } else {
                    int_cbrt(self)
                }
            }
        }

        impl IRoot<2> for $type {
            #[inline(always)]
            fn iroot(self) -> Self {
                TruncRoot::<2>::trunc_root(self)
            }
        }
        impl IRoot<3> for $type {
            #[inline(always)]
            fn iroot(self) -> Self {
                TruncRoot::<3>::trunc_root(self)
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
        let mut low = crate_int!(1);
        let mut high = value;

        while low <= high {
            let mid = (low + high) / crate_int!(2);
            let square = mid * mid;

            if square == value {
                return mid;
            } else if square < value {
                low = mid + crate_int!(1);
            } else {
                high = mid - crate_int!(1);
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
        let mut low = crate_int!(1);
        let mut high = value;

        while low <= high {
            let mid = (low + high) / crate_int!(2);
            let square = mid * mid * mid;

            if square == value {
                return mid;
            } else if square < value {
                low = mid + crate_int!(1);
            } else {
                high = mid - crate_int!(1);
            }
        }

        high
    }
}
