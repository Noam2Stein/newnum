use numflex::*;

fn main() {
    println!("{}", test(5, 2));
}

fn test<T: Num + Two>(a: T, b: T) -> T {
    a + b.floor() * T::TWO
}
