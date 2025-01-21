use crate::{Floating, Positive, Zero};

use super::{FullySigned, Normalizable};

macro_rules! from_trait {
    ($trait:ident($fn:ident $type:ident) $(: $first_supertrait:ident $( + $supertrait:ident)*)? { impl for $($impl_type:ident)* }) => {
        pub trait $trait $(: $first_supertrait $( + $supertrait)*)? {
            fn $fn(value: $type) -> Self;
        }

        $(
            impl $trait for $impl_type {
                fn $fn(value: $type) -> Self {
                    value as _
                }
            }
        )*
    };
}

from_trait!(FromU7(u7 u8): Zero + Positive { impl for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 });
from_trait!(FromI2(i2 i8): FullySigned + Normalizable { impl for i8 i16 i32 i64 i128 isize f32 f64 });

from_trait!(FromU8(u8 u8): Zero + Positive { impl for u8 u16 u32 u64 u128 usize i16 i32 i64 i128 isize f32 f64 });
from_trait!(FromU16(u16 u16): FromU8 { impl for u16 u32 u64 u128 usize i32 i64 i128 isize f32 f64 });
from_trait!(FromU32(u32 u32): FromU16 { impl for u32 u64 u128 usize i64 i128 f64 });
from_trait!(FromU64(u64 u64): FromU32 { impl for u64 u128 i128 });
from_trait!(FromU128(u128 u128): FromU64 { impl for u128 });

from_trait!(FromI8(i8 i8): FromU7 + FromI2 { impl for i8 i16 i32 i64 i128 isize f32 f64 });
from_trait!(FromI16(i16 i16): FromI8 + FromU8 { impl for i16 i32 i64 i128 isize f32 f64 });
from_trait!(FromI32(i32 i32): FromI16 + FromU16 { impl for i32 i64 i128 isize f64 });
from_trait!(FromI64(i64 i64): FromI32 + FromU32 { impl for i64 i128 });
from_trait!(FromI128(i128 i128): FromI64 + FromU64 { impl for i128 });

from_trait!(FromF32(f32 f32): FromU16 + FromI16 + Floating { impl for f32 f64 });
from_trait!(FromF64(f64 f64): FromU32 + FromI32 + Floating { impl for f64 });
