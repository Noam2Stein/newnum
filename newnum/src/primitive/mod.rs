use std::fmt::{Debug, Display};

use crate::{IRoot, IRound, Num, Positive, Signum, TruncRoot, WholeEquivalent, Zero};

mod float;
mod int;
mod signed;
mod sint;
mod uint;
mod unsigned;
pub use float::*;
pub use int::*;
pub use signed::*;
pub use sint::*;
pub use uint::*;
pub use unsigned::*;

/// Trait for number primitives (```u8```, ```i32```, ```f64```...).
///
/// Can be implemented by non ```std``` types that follow the primitive rules:
/// * Can represent 0..=127
/// * ```Default``` returns ```0```
/// * implements ```Send + Sync + Debug + Display + Copy```
/// * Can be failibly converted to any std primitive
pub trait Prim:
    Num
    + Positive
    + Zero
    + Signum
    + AsPrim
    + WholeEquivalent
    + TruncRoot<2>
    + TruncRoot<3>
    + IRoot<2>
    + IRoot<3>
    + IRound
    + Send
    + Sync
    + Debug
    + Display
    + Clone
    + Copy
    + Default
{
    fn from_num<T: AsPrim>(value: T) -> Self;
}

macro_rules! code_for_prims {
    ($($type:ident($as_fn:ident),)*) => {
        code_for_prims!($ident $($type($as_fn),)*);
    };
    ($dollar:tt $_:ident $($type:ident($as_fn:ident),)*) => {
        pub trait AsPrim: Sized {
            $(
                fn $as_fn(self) -> $type;
            )*

            fn as_num<T: Prim>(self) -> T {
                T::from_num(self)
            }
        }

        $(
            impl Prim for $type {
                #[inline(always)]
                fn from_num<T: AsPrim>(value: T) -> Self {
                    value.$as_fn()
                }
            }
        )*

        macro_rules! impl_as_Prims {
            ($dollar($type2:ident($as_fn2:ident),)*) => {$(
                impl AsPrim for $type {
                    $dollar(
                        fn $as_fn2(self) -> $type2 {
                            self as _
                        }
                    )*
                }
            )*}
        }
        impl_as_Prims!($($type($as_fn),)*);
    };
}
code_for_prims!(
    u8(as_u8),
    u16(as_u16),
    u32(as_u32),
    u64(as_u64),
    u128(as_u128),
    usize(as_usize),
    i8(as_i8),
    i16(as_i16),
    i32(as_i32),
    i64(as_i64),
    i128(as_i128),
    isize(as_isize),
    f32(as_f32),
    f64(as_f64),
);
