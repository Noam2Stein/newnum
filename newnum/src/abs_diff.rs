use std::ops::Sub;

pub trait AbsDiff: Sized + Sub {
    fn abs_diff(self, rhs: Self) -> Self::Output;
}

macro_rules! impl_abs_diff_for_uints {
    ($($type:ty)*) => {
        $(
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
        )*
    };
}
impl_abs_diff_for_uints!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_abs_diff_for_sints {
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
impl_abs_diff_for_sints!(i8 i16 i32 i64 i128 isize f32 f64);
