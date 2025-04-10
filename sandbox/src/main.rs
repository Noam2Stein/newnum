use newnum::*;

fn main() {
    println!("{}", Fun(0).is_bin_positive())
}

#[derive(Sign, Round)]
#[derive_bound(Sign; T: Sign<BoolMapped = bool>)]
struct Fun<T: Round>(T);

#[derive(Sign, Round)]
enum _Fun2 {
    A { a: i8 },
    B(i8),
}

#[derive(Round, Trig, ATrig, Hyper, AHyper)]
struct Fun3(f32, f32);
