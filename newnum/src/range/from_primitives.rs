use crate::{Floating, Num, Positive, Zero};

use super::{FullySigned, Normalizable};

macro_rules! from_trait {
    ($(#[$meta:meta])* $trait:ident($fn:ident $type:ident) $(: $first_supertrait:ident $( + $supertrait:ident)*)? { impl for $($impl_type:ident)* }) => {
        $(#[$meta])*
        pub trait $trait: Num $(+ $first_supertrait $( + $supertrait)*)? {
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

from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=127`.
    FromU7(u7 u8): Zero + Positive { impl for u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent both `1`, `-1` and `0`.
    FromI2(i2 i8): FullySigned + Normalizable { impl for i8 i16 i32 i64 i128 isize f32 f64 }
);

from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=255`.
    FromU8(u8 u8): Zero + Positive { impl for u8 u16 u32 u64 u128 usize i16 i32 i64 i128 isize f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=65_535`.
    FromU16(u16 u16): FromU8 { impl for u16 u32 u64 u128 usize i32 i64 i128 isize f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=4_294_967_295`.
    FromU32(u32 u32): FromU16 { impl for u32 u64 u128 usize i64 i128 f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=18_446_744_073_709_551_615`.
    FromU64(u64 u64): FromU32 { impl for u64 u128 i128 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `0..=340_282_366_920_938_463_463_374_607_431_768_211_455`.
    FromU128(u128 u128): FromU64 { impl for u128 }
);

from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `-128..=127`.
    FromI8(i8 i8): FromU7 + FromI2 { impl for i8 i16 i32 i64 i128 isize f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `-32_768..=32_767`.
    FromI16(i16 i16): FromI8 + FromU8 { impl for i16 i32 i64 i128 isize f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `-2_147_483_648..=2_147_483_647`.
    FromI32(i32 i32): FromI16 + FromU16 { impl for i32 i64 i128 isize f64 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `-9_223_372_036_854_775_808..=9_223_372_036_854_775_807`.
    FromI64(i64 i64): FromI32 + FromU32 { impl for i64 i128 }
);
from_trait!(
    /// Trait for number types that can represent all whole numbers in the range `-170_141_183_460_469_231_731_687_303_715_884_105_728..=170_141_183_460_469_231_731_687_303_715_884_105_727`.
    FromI128(i128 i128): FromI64 + FromU64 { impl for i128 }
);

from_trait!(
    /// Trait for number types that can represent all `f32` values.
    ///
    /// Meaning all whole numbers in the range `-2^23..=2^23-1` which is `-8_388_608..=8_388_607`,
    /// and also the floating numbers that `f32` supports (in the range `-3.4028235e38..=3.4028235e38`).
    FromF32(f32 f32): FromU16 + FromI16 + Floating { impl for f32 f64 }
);
from_trait!(
    /// Trait for number types that can represent all `f64` values.
    ///
    /// Meaning all whole numbers in the range `-2^52..=2^52-1` which is `-4_503_599_627_370_496..=4_503_599_627_370_495`,
    /// and also the floating-point numbers that `f64` supports (in the range `-1.7976931348623157e308..=1.7976931348623157e308`).
    FromF64(f64 f64): FromU32 + FromI32 + Floating { impl for f64 }
);
