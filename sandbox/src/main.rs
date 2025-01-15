use newnum::*;

fn main() {
    println!("{}", test(5.0, 2.5));
}

fn test<T: Num + FromU7>(a: T, b: T) -> T {
    a + b * T::u7(2)
}
