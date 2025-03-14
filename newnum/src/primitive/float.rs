use crate::{AHyper, ATrig, Hyper, Root, Trig};

use super::SignedPrim;

pub trait Float:
    SignedPrim
    + Root<2>
    + Root<3>
    + Trig<Output = Self>
    + ATrig<Output = Self>
    + Hyper<Output = Self>
    + AHyper<Output = Self>
{
}

impl Float for f32 {}
impl Float for f64 {}
