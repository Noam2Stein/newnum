#![allow(dead_code)]

use angle::Angle;
use newnum::{ATrig, Trig};

mod angle;
mod ratio;
mod whole_ratio;

fn main() {
    println!("{}", Angle::from_degrees(30).cos().acos())
}
