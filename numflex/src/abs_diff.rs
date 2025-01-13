pub trait AbsDiff<Rhs = Self> {
    type Output;

    fn abs_diff(self, rhs: Rhs) -> Self::Output;
}

macro_rules! impl_for_unsigned {
    ($($type:ty)*) => {
        $(
            impl AbsDiff for $type {
                type Output = Self;

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
impl_for_unsigned!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_for_signed {
    ($($type:ty)*) => {
        $(
            impl AbsDiff for $type {
                type Output = Self;

                #[inline(always)]
                fn abs_diff(self, rhs: Self) -> Self::Output {
                    (self - rhs).abs()
                }
            }
        )*
    };
}
impl_for_signed!(i8 i16 i32 i64 i128 isize f32 f64);
