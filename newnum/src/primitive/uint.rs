use super::{Int, UnsignedPrim};

pub trait UInt: Int + UnsignedPrim {}

impl UInt for u8 {}
impl UInt for u16 {}
impl UInt for u32 {}
impl UInt for u64 {}
impl UInt for u128 {}
impl UInt for usize {}
