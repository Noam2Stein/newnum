use std::ops::Sub;

/// Trait for `abs_diff` method, which computes the absolute difference between two numbers.
/// `abs_diff` is equivalent to `(a - b).abs()`, not `a.abs() - b.abs()`.
///
/// This trait is generic over `Rhs` which is default to `Self`, like multi-side operator traits.
/// It is required that `Self` is `Sub<Rhs>` and `Rhs` is `Sub<Self>` where they both have the same output type.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// fn main() {
///     assert(10.abs_diff(20) == 10);
///     assert((-10).abs_diff(20) == 30);
/// }
/// ```
pub trait AbsDiff<Rhs: Sub<Self, Output = Self::Output> = Self>: Sized + Sub<Rhs> {
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
