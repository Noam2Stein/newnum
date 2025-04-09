use newnum::*;

fn main() {
    println!("{}", Fun(0).is_bin_positive())
}

#[derive(Sign)]
#[derive_bound(Sign; T: Sign<BoolMapped = bool>)]
struct Fun<T>(T);
