use std::ops::Sub;

pub trait AbsDiff: Sized + Sub {
    fn abs_diff(self, rhs: Self) -> Self::Output;
}

macro_rules! uint_impl {
    ($type:ident) => {
        impl AbsDiff for $type {
            #[inline(always)]
            fn abs_diff(self, rhs: Self) -> Self::Output {
                if self > rhs {
                    self - rhs
                } else {
                    rhs - self
                }
            }
        }
    };
}
uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(u128);
uint_impl!(usize);

macro_rules! signed_impl {
    ($($type:ty)*) => {
        $(
            impl AbsDiff for $type {
                #[inline(always)]
                fn abs_diff(self, rhs: Self) -> Self::Output {
                    (self - rhs).abs()
                }
            }
        )*
    };
}
signed_impl!(i8);
signed_impl!(i16);
signed_impl!(i32);
signed_impl!(i64);
signed_impl!(i128);
signed_impl!(isize);
signed_impl!(f32);
signed_impl!(f64);
