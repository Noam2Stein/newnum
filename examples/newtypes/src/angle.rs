use std::{
    fmt::{Debug, Display},
    ops::{Div, DivAssign, Mul, MulAssign},
};

use derive_more::derive::{Add, AddAssign, Rem, RemAssign, Sub, SubAssign};
use newnum::{Prim, Trig};

use crate::ratio::Ratio;

#[derive(
    Clone, Copy, PartialEq, PartialOrd, Default, Add, AddAssign, Sub, SubAssign, Rem, RemAssign,
)]
pub struct Angle(f64);

impl Angle {
    pub fn from_degrees(degrees: impl Prim) -> Self {
        Self(degrees.as_f64().to_radians())
    }
    pub fn from_radians(radians: impl Prim) -> Self {
        Self(radians.as_f64())
    }

    pub fn to_degrees(self) -> f64 {
        self.0.to_degrees()
    }
    pub fn to_radians(self) -> f64 {
        self.0
    }
}

impl Display for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} deg)", self.0.to_degrees())
    }
}
impl Debug for Angle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} deg)", self.0.to_degrees())
    }
}

impl Mul<f64> for Angle {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Div<f64> for Angle {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}
impl MulAssign<f64> for Angle {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs
    }
}
impl DivAssign<f64> for Angle {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs
    }
}

impl Trig for Angle {
    type Output = Ratio;

    fn sin(self) -> Self::Output {
        Ratio(self.0.sin())
    }

    fn cos(self) -> Self::Output {
        Ratio(self.0.cos())
    }

    fn tan(self) -> Self::Output {
        Ratio(self.0.tan())
    }
}
