use newnum::*;

fn main() {
    let mut i = 5.0;
    mul_by_two(&mut i);
    println!("{i}")
}

fn mul_by_two(value: &mut impl Num) {
    *value *= float!(1.5)
}
