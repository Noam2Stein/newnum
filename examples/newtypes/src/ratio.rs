use std::fmt::Display;

use derive_more::derive::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign,
};
use newnum::{ATrig, IRound, WholeEquivalent};

use crate::{angle::Angle, whole_ratio::WholeRatio};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[derive(Add, Sub, Mul, Div, Rem)]
#[derive(AddAssign, SubAssign, MulAssign, DivAssign, RemAssign)]
pub struct Ratio(pub f64);

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (numerator, denominator) = self.to_numerator_denominator();

        write!(f, "{numerator} / {denominator}")
    }
}

impl WholeEquivalent for Ratio {
    type Whole = WholeRatio;
}

impl IRound for Ratio {
    fn iround(self) -> Self::Whole {
        WholeRatio(self.0.iround())
    }
    fn ifloor(self) -> Self::Whole {
        WholeRatio(self.0.ifloor())
    }
    fn iceil(self) -> Self::Whole {
        WholeRatio(self.0.iceil())
    }
    fn itrunc(self) -> Self::Whole {
        WholeRatio(self.0.itrunc())
    }
    fn iatrunc(self) -> Self::Whole {
        WholeRatio(self.0.iatrunc())
    }
}

impl ATrig for Ratio {
    type Output = Angle;

    fn asin(self) -> Self::Output {
        Angle::from_radians(self.0.asin())
    }

    fn acos(self) -> Self::Output {
        Angle::from_radians(self.0.acos())
    }

    fn atan(self) -> Self::Output {
        Angle::from_radians(self.0.atan())
    }
}

impl Ratio {
    pub fn to_numerator_denominator(self) -> (i64, i64) {
        let mut num = self.0;
        let mut a = num.floor() as i64;
        let mut numerator = a;
        let mut denominator = 1;
        let mut prev_numerator = 1;
        let mut prev_denominator = 0;

        while (self.0 - numerator as f64 / denominator as f64).abs() > f64::EPSILON {
            num = 1.0 / (num - a as f64);
            a = num.floor() as i64;

            let next_numerator = a * numerator + prev_numerator;
            let next_denominator = a * denominator + prev_denominator;

            if next_denominator > 999999999 {
                break;
            }

            prev_numerator = numerator;
            prev_denominator = denominator;
            numerator = next_numerator;
            denominator = next_denominator;
        }

        (numerator, denominator)
    }
}
