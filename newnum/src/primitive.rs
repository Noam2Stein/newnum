use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul,
        MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    },
};

use crate::{
    ATrig, ATrigH, Cbrt, FromI8, FromU7, FromU8, Negative, NotNegative, Num, Positive, Sqrt, Trig,
    TrigH, Whole, Zero,
};

macro_rules! code_for_Prims {
    ($($type:ident($as_fn:ident),)*) => {
        code_for_Prims!($ident $($type($as_fn),)*);
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

        pub trait Prim:
            Num
            + FromU7
            + Zero
            + Positive
            + AsPrim
            + Mul<Output = Self>
            + Div<Output = Self>
            + Rem<Output = Self>
            + MulAssign
            + DivAssign
            + RemAssign
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
code_for_Prims!(
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

pub trait Int:
    Prim
    + Whole
    + Not<Output = Self>
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + ShlAssign
    + ShrAssign
    + BitAndAssign
    + BitOrAssign
    + BitXorAssign
    + Hash
    + Eq
    + Ord
{
}

pub trait SignedPrim: Prim + Negative + Neg<Output = Self> + FromI8 {}
pub trait UnsignedPrim: Prim + NotNegative + FromU8 {}

pub trait SInt: Int + SignedPrim {}
pub trait UInt: Int + UnsignedPrim {}
pub trait Float:
    SignedPrim
    + Sqrt<Output = Self>
    + Cbrt<Output = Self>
    + Trig<Output = Self>
    + ATrig<Output = Self>
    + TrigH<Output = Self>
    + ATrigH<Output = Self>
{
}

impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}
impl UInt for u128 {}
impl UInt for usize {}
impl SInt for i8 {}
impl SInt for i16 {}
impl SInt for i32 {}
impl SInt for i64 {}
impl SInt for i128 {}
impl SInt for isize {}
impl Float for f32 {}
impl Float for f64 {}

impl Int for u8 {}
impl Int for u16 {}
impl Int for u32 {}
impl Int for u64 {}
impl Int for u128 {}
impl Int for usize {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
impl Int for isize {}

impl SignedPrim for i8 {}
impl SignedPrim for i16 {}
impl SignedPrim for i32 {}
impl SignedPrim for i64 {}
impl SignedPrim for i128 {}
impl SignedPrim for isize {}
impl SignedPrim for f32 {}
impl SignedPrim for f64 {}

impl UnsignedPrim for u8 {}
impl UnsignedPrim for u16 {}
impl UnsignedPrim for u32 {}
impl UnsignedPrim for u64 {}
impl UnsignedPrim for u128 {}
impl UnsignedPrim for usize {}
