use newnum::*;

fn main() {
    println!("{}", test(5.0, 2.5));
}

fn test<T: Num + FromU8>(a: T, b: T) -> T {
    a + b * T::u8(2)
}
