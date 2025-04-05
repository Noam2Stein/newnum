use crate::*;

pub trait SignedPrim: Prim + Negative + Neg<Output = Self> {}

impl SignedPrim for i8 {}
impl SignedPrim for i16 {}
impl SignedPrim for i32 {}
impl SignedPrim for i64 {}
impl SignedPrim for i128 {}
impl SignedPrim for isize {}
impl SignedPrim for f32 {}
impl SignedPrim for f64 {}
