use newnum::*;

fn main() {
    println!("{}", guf(0, -2))
}

fn guf<T: Prim>(a: T, b: T) -> T {
    a.signumf() * b.signumf()
}
