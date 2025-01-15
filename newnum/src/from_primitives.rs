macro_rules! from_trait {
    ($trait:ident($type:ident) $(: $first_supertrait:ident $( + $supertrait:ident)*)? for $($impl_type:ident)*) => {
        pub trait $trait $(: $first_supertrait $( + $supertrait)*)? {
            fn $type(value: $type) -> Self;
        }

        $(
            impl $trait for $impl_type {
                fn $type(value: $type) -> Self {
                    value as _
                }
            }
        )*
    };
}

from_trait!(FromU8(u8) for u8 u16 u32 u64 u128 usize i16 i32 i64 i128 isize f32 f64);
from_trait!(FromU16(u16): FromU8 for u16 u32 u64 u128 usize i32 i64 i128 isize f32 f64);
from_trait!(FromU32(u32): FromU16 for u32 u64 u128 usize i64 i128 f64);
from_trait!(FromU64(u64): FromU32 for u64 u128 i128);
from_trait!(FromU128(u128): FromU64 for u128);

from_trait!(FromI8(i8) for i8 i16 i32 i64 i128 isize f32 f64);
from_trait!(FromI16(i16): FromI8 + FromU8 for i16 i32 i64 i128 isize f32 f64);
from_trait!(FromI32(i32): FromI16 + FromU16 for i32 i64 i128 isize f64);
from_trait!(FromI64(i64): FromI32 + FromU32 for i64 i128);
from_trait!(FromI128(i128): FromI64 + FromU64 for i128);

from_trait!(FromF32(f32): FromU16 + FromI16 for f32 f64);
from_trait!(FromF64(f64): FromU32 + FromI32 for f64);
