use crate::*;

pub trait SInt: Int + SignedPrim {}

impl SInt for i8 {}
impl SInt for i16 {}
impl SInt for i32 {}
impl SInt for i64 {}
impl SInt for i128 {}
impl SInt for isize {}
