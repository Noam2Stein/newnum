use std::{fmt::Display, ops::Add};

use derive_more::derive::{Add, AddAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use newnum::{ATrig, IRound, Round, WholeEquivalent};

use crate::{angle::Angle, ratio::Ratio};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(Add, Sub, Mul, Rem)]
#[derive(AddAssign, SubAssign, MulAssign, RemAssign)]
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

impl WholeEquivalent for WholeRatio {
    type Whole = Self;
}
impl Round for WholeRatio {
    fn round(self) -> Self {
        self
    }
    fn floor(self) -> Self {
        self
    }
    fn ceil(self) -> Self {
        self
    }
    fn trunc(self) -> Self {
        self
    }
    fn atrunc(self) -> Self {
        self
    }
}

impl IRound for WholeRatio {
    fn iround(self) -> Self::Whole {
        self
    }
    fn ifloor(self) -> Self::Whole {
        self
    }
    fn iceil(self) -> Self::Whole {
        self
    }
    fn iatrunc(self) -> Self::Whole {
        self
    }
    fn itrunc(self) -> Self::Whole {
        self
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
