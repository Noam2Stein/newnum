use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

mod abs_diff;
mod from_primitives;
mod primitive;
mod root;
mod round;
mod sign;
mod trig;
pub use abs_diff::*;
pub use from_primitives::*;
pub use primitive::*;
pub use root::*;
pub use round::*;
pub use sign::*;
pub use trig::*;

pub trait Num:
    AbsDiff<Output = Self>
    + Sqrt<Output = Self>
    + Cbrt<Output = Self>
    + Round
    + MaybeSigned
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
