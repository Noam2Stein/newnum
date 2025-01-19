use crate::{ATrig, ATrigH, Root, Trig, TrigH};

use super::SignedPrim;

pub trait Float:
    SignedPrim
    + Root
    + Trig<Output = Self>
    + ATrig<Output = Self>
    + TrigH<Output = Self>
    + ATrigH<Output = Self>
{
}

impl Float for f32 {}
impl Float for f64 {}
