macro_rules! from_trait {
    ($trait:ident($type:ident) $(: $first_supertrait:ident $( + $supertrait:ident)*)?) => {
        pub trait $trait $(: $first_supertrait $( + $supertrait)*)? {
            fn $type(value: $type) -> Self;
        }
    };
}

from_trait!(FromU8(u8));
from_trait!(FromU16(u16): FromU8);
from_trait!(FromU32(u32): FromU16);
from_trait!(FromU64(u64): FromU32);
from_trait!(FromU128(u128): FromU64);

from_trait!(FromI8(i8));
from_trait!(FromI16(i16): FromI8 + FromU8);
from_trait!(FromI32(i32): FromI16 + FromU16);
from_trait!(FromI64(i64): FromI32 + FromU32);
from_trait!(FromI128(i128): FromI64 + FromU64);

from_trait!(FromF32(f32): FromU16 + FromI16);
from_trait!(FromF64(f64): FromU32 + FromI32);
