use derive_more::*;
use newnum::derive::*;

fn main() {
    println!("Out Of Sand")
}

#[derive(Sub, AbsDiff, MinMax)]
struct Test(u8, u16, u32);

#[derive(MinMax, PartialEq, PartialOrd)]
#[flat_minmax]
enum _TheFuse94Dot7 {
    A(i8),
    B(u16),
}
