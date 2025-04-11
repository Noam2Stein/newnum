use derive_more::*;
use newnum::derive::*;

fn main() {
    println!("Out Of Sand")
}

#[derive(Sub, AbsDiff, TruncRoot)]
struct Test(u8, u16, u32);
