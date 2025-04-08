use crate::*;

pub trait Float:
    FromFloatLiteral
    + SignedPrim
    + Root
    + Trig<Output = Self>
    + ATrig<Output = Self>
    + Hyper<Output = Self>
    + AHyper<Output = Self>
{
}

impl Float for f32 {}
impl Float for f64 {}
