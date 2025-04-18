#[allow(unused_imports)]
use derive_more::*;
#[allow(unused_imports)]
use newnum::{derive::*, *};

fn main() {
    println!("{}", convert(453_u128))
}

fn convert(a: impl Prim) -> i32 {
    a.as_num()
}
