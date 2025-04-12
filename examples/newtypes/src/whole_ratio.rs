use std::{fmt::Display, ops::Add};

use derive_more::derive::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use newnum::{derive::*, *};

use crate::{angle::Angle, ratio::Ratio};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(Add, Sub, Mul, Rem)]
#[derive(AddAssign, SubAssign, MulAssign, RemAssign)]
#[derive(Whole)]
pub struct WholeRatio(pub i64);

impl Display for WholeRatio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} / 1", self.0)
    }
}

impl Add<Ratio> for WholeRatio {
    type Output = Ratio;

    fn add(self, rhs: Ratio) -> Self::Output {
        Ratio(self.0 as f64 + rhs.0)
    }
}

impl ATrig for WholeRatio {
    type Output = Angle;

    fn asin(self) -> Self::Output {
        Angle::from_radians((self.0 as f64).asin())
    }

    fn acos(self) -> Self::Output {
        Angle::from_radians((self.0 as f64).acos())
    }

    fn atan(self) -> Self::Output {
        Angle::from_radians((self.0 as f64).atan())
    }
}
