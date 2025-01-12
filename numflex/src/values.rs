use crate::{Negative, Positive};

macro_rules! value_trait {
    ($trait:ident $(: $first_supertrait:ident $( + $supertrait:ty)*)? { $const:ident = -$int:literal }) => {
        value_trait!(for { i8 i16 i32 i64 i128 isize f32 f64 } $trait $(: $first_supertrait $( + $supertrait)*)? { $const = -$int });
    };
    ($trait:ident $(: $first_supertrait:ident $( + $supertrait:ty)*)? { $const:ident = $int:literal }) => {
        value_trait!(for { u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64 } $trait $(: $first_supertrait $( + $supertrait)*)? { $const = $int });
    };
    (for { $($type:ty)* } $trait:ident $(: $first_supertrait:ident $( + $supertrait:ty)*)? { $const:ident = $int:expr }) => {
        pub trait $trait $(: $first_supertrait $( + $supertrait)*)? {
            const $const: Self;
        }
        $(
            impl $trait for $type {
                const $const: Self = $int as _;
            }
        )*
    };
}
value_trait!(Zero { ZERO = 0 });
value_trait!(One: Positive { ONE = 1 });
value_trait!(Two: Positive { TWO = 2 });
value_trait!(Three: Positive { THREE = 3 });
value_trait!(NegOne: Negative { NEG_ONE = -1 });
value_trait!(NegTwo: Negative { NEG_TWO = -2 });
value_trait!(NegThree: Negative { NEG_THREE = -3 });
