use crate::{IRound, Round};

/// Trait for numbers or number containers that are always round (for example ```i32```).
pub trait Whole: WholeEquivalent<Whole = Self> + Round + IRound {}

impl<T: WholeEquivalent<Whole = Self> + Round + IRound> Whole for T {}

/// Trait for numbers/number-containers that have whole/integer equivilants.
/// The `Whole` type should be the lightest whole type that can represent the entire range of `Self`.
pub trait WholeEquivalent {
    type Whole: Whole;

    fn whole(self) -> Self::Whole;
}

macro_rules! int_impl {
    ($type:ident) => {
        impl WholeEquivalent for $type {
            type Whole = Self;

            #[inline(always)]
            fn whole(self) -> Self::Whole {
                self
            }
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

macro_rules! float_impl {
    ($type:ident => $whole:ident) => {
        impl WholeEquivalent for $type {
            type Whole = $whole;

            #[inline(always)]
            fn whole(self) -> Self::Whole {
                self as _
            }
        }
    };
}
float_impl!(f32 => i32);
float_impl!(f64 => i64);
