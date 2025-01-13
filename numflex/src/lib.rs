use std::ops::{Add, AddAssign, Sub, SubAssign};

mod abs_diff;
mod primitive;
mod root;
mod round;
mod sign;
mod trig;
mod values;
pub use abs_diff::*;
pub use primitive::*;
pub use root::*;
pub use round::*;
pub use sign::*;
pub use trig::*;
pub use values::*;

pub trait Num:
    MaybeSigned
    + Round
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + AddAssign
    + SubAssign
    + AbsDiff<Output = Self>
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
