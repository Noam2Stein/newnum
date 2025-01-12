pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

pub trait Cbrt {
    type Output;

    fn cbrt(self) -> Self::Output;
}

macro_rules! impl_for_floats {
    ($($type:ty)*) => {
        $(
            impl Sqrt for $type {
                type Output = Self;

                #[inline(always)]
                fn sqrt(self) -> Self::Output {
                    self.sqrt()
                }
            }

            impl Cbrt for $type {
                type Output = Self;

                #[inline(always)]
                fn cbrt(self) -> Self::Output {
                    self.cbrt()
                }
            }
        )*
    };
}
impl_for_floats!(f32 f64);

macro_rules! impl_for_ints {
    ($($type:ident($float:ident))*) => {
        $(
            impl Sqrt for $type {
                type Output = Self;

                #[inline(always)]
                fn sqrt(self) -> Self::Output {
                    (self as $float).sqrt() as _
                }
            }

            impl Cbrt for $type {
                type Output = Self;

                #[inline(always)]
                fn cbrt(self) -> Self::Output {
                    (self as $float).cbrt() as _
                }
            }
        )*
    };
}
impl_for_ints!(u8(f32) u16(f32) u32(f64) u64(f64) u128(f64) usize(f64));
impl_for_ints!(i8(f32) i16(f32) i32(f64) i64(f64) i128(f64) isize(f64));
