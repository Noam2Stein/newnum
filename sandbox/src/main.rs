use newnum::*;

fn main() {}

struct Angle;
struct Ratio;

impl Trig for Angle {
    type Output = Ratio;

    fn sin(self) -> Self::Output {
        Ratio
    }
    fn cos(self) -> Self::Output {
        Ratio
    }
    fn tan(self) -> Self::Output {
        Ratio
    }
    fn cot(self) -> Self::Output {
        Ratio
    }
}
impl ATrig for Ratio {
    type Output = Angle;

    fn asin(self) -> Self::Output {
        Angle
    }
    fn acos(self) -> Self::Output {
        Angle
    }
    fn atan(self) -> Self::Output {
        Angle
    }
    fn acot(self) -> Self::Output {
        Angle
    }
}
