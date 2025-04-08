use std::ops::*;

mod api;
mod initialization;
mod primitive;
mod range;
pub use api::*;
pub use initialization::*;
pub use primitive::*;
pub use range::*;

pub use newnum_proc_macros::{
    num, num_approx, Float, Int, Num, Prim, SInt, SignedPrim, UInt, UnsignedPrim,
};

#[allow(unused_imports)]
use newnum_proc_macros::{internal_num, internal_num_approx};

/// Trait for types that represent abstract numbers.
/// Requires ```+-*/%=<>``` opeartors.
///
/// can be implemented by non primitives! for a primitives only trait use [```Prim```].
///
/// Can't nessesarely represent ```0```, ```1``` or any common value.
/// For basic positive values add ``` + FromU8```,
/// for negative values as well add ``` + FromI8```.
pub trait Num:
    FromIntLiteral
    + AbsDiff<Output = Self>
    + MinMax
    + TruncRoot
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
}

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Num for isize {}

impl Num for f32 {}
impl Num for f64 {}
