use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

mod api;
mod primitive;
mod range;
pub use api::*;
pub use primitive::*;
pub use range::*;

pub mod derive;

pub use newnum_proc_macros::{float, int};

/// Trait for types that represent abstract numbers.
/// Requires ```+-*/%=<>``` opeartors.
///
/// can be implemented by non primitives! for a primitives only trait use [```Prim```].
///
/// Can't nessesarely represent ```0```, ```1``` or any common value.
/// For basic positive values add ``` + FromU8```,
/// for negative values as well add ``` + FromI8```.
pub trait Num:
    AbsDiff<Output = Self>
    + MinMax
    + TruncRoot<2, Root = Self>
    + TruncRoot<3, Root = Self>
    + Round
    + Sign<BoolMapped = bool>
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
    const MIN_CONSTANT: Option<i128>;
    const MAX_CONSTANT: Option<i128>;
    const ALLOW_FLOAT_CONSTANTS: bool;

    unsafe fn int_constant(value: i128) -> Self;
    unsafe fn float_constant(value: f64) -> Self;
}

macro_rules! int_impl {
    ($ty:ty) => {
        impl Num for $ty {
            const MIN_CONSTANT: Option<i128> = Some(Self::MIN as _);
            const MAX_CONSTANT: Option<i128> = Some(Self::MAX as _);
            const ALLOW_FLOAT_CONSTANTS: bool = false;

            unsafe fn int_constant(value: i128) -> Self {
                value as Self
            }
            unsafe fn float_constant(_value: f64) -> Self {
                unreachable!()
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
    ($ty:ty) => {
        impl Num for $ty {
            const MIN_CONSTANT: Option<i128> = Some(Self::MIN as _);
            const MAX_CONSTANT: Option<i128> = Some(Self::MAX as _);
            const ALLOW_FLOAT_CONSTANTS: bool = true;

            unsafe fn int_constant(value: i128) -> Self {
                value as Self
            }
            unsafe fn float_constant(value: f64) -> Self {
                value as Self
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
