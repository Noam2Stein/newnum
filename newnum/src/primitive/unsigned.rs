use crate::{FromU8, MinValue, NotNegative};

use super::Prim;

pub trait UnsignedPrim: Prim + NotNegative + FromU8 + MinValue {}

impl UnsignedPrim for u8 {}
impl UnsignedPrim for u16 {}
impl UnsignedPrim for u32 {}
impl UnsignedPrim for u64 {}
impl UnsignedPrim for u128 {}
impl UnsignedPrim for usize {}
