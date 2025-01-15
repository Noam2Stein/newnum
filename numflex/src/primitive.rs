use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Mul,
        MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign,
    },
};

use crate::{
    ATrig, ATrigH, Cbrt, FromI8, FromU8, NegOne, NegThree, NegTwo, Negative, NotNegative, Num, One,
    Positive, Sqrt, Three, Trig, TrigH, Two, Whole, Zero,
};

macro_rules! code_for_primitives {
    ($($type:ident($as_fn:ident),)*) => {
        code_for_primitives!($ident $($type($as_fn),)*);
    };
    ($dollar:tt $_:ident $($type:ident($as_fn:ident),)*) => {
        pub trait AsPrimitive: Sized {
            $(
                fn $as_fn(self) -> $type;
            )*

            fn as_num<T: Pri>(self) -> T {
                T::from_num(self)
            }
        }

        pub trait Pri:
            Num
            + Positive
            + Zero
            + One
            + Two
            + Three
            + AsPrimitive
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
            fn from_num<T: AsPrimitive>(value: T) -> Self;
        }

        $(
            impl Pri for $type {
                #[inline(always)]
                fn from_num<T: AsPrimitive>(value: T) -> Self {
                    value.$as_fn()
                }
            }
        )*

        macro_rules! impl_as_primitives {
            ($dollar($type2:ident($as_fn2:ident),)*) => {$(
                impl AsPrimitive for $type {
                    $dollar(
                        fn $as_fn2(self) -> $type2 {
                            self as _
                        }
                    )*
                }
            )*}
        }
        impl_as_primitives!($($type($as_fn),)*);
    };
}
code_for_primitives!(
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
    Pri
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

pub trait SignedPrimitive:
    Pri + Negative + NegOne + NegTwo + NegThree + Neg<Output = Self> + FromI8
{
}
pub trait UnsignedPrimitive: Pri + NotNegative + FromU8 {}

pub trait SInt: Int + SignedPrimitive {}
pub trait UInt: Int + UnsignedPrimitive {}
pub trait Float:
    SignedPrimitive
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

impl SignedPrimitive for i8 {}
impl SignedPrimitive for i16 {}
impl SignedPrimitive for i32 {}
impl SignedPrimitive for i64 {}
impl SignedPrimitive for i128 {}
impl SignedPrimitive for isize {}
impl SignedPrimitive for f32 {}
impl SignedPrimitive for f64 {}

impl UnsignedPrimitive for u8 {}
impl UnsignedPrimitive for u16 {}
impl UnsignedPrimitive for u32 {}
impl UnsignedPrimitive for u64 {}
impl UnsignedPrimitive for u128 {}
impl UnsignedPrimitive for usize {}
