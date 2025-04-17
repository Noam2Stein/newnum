use std::ops::*;

mod api;
mod initialization;
mod primitive;
pub use api::*;
pub use initialization::*;
pub use primitive::*;

pub use newnum_proc_macros::{num, num_approx};

/// Derive macros for crate traits.
pub mod derive {
    pub use newnum_proc_macros::{
        AHyper, ATrig, AbsDiff, AlwaysNegative, AlwaysPositive, AlwaysZero, FullySigned, Hyper,
        MinMax, Negative, NegativeOrZero, NotNegative, NotPositive, NotZero, Num, Positive,
        PositiveOrNegative, PositiveOrZero, Root, Round, Signed, Trig, TruncRoot, TypeMax, TypeMin,
        Whole, Zero,
    };
}

#[allow(unused_imports)]
use newnum_proc_macros::{internal_num, internal_num_approx};

/// Trait for types that represent numbers.
///
/// This includes support for the common arithmetic and comparison operators:
/// `+`, `-`, `*`, `/`, `%`, `=`, `<`, `>`, as well as traits for absolute
/// difference, min/max, rounding, and sign detection.
///
/// This trait can be implemented by non-primitive types, such as unit-based
/// numbers (`Meters`, `Seconds`, etc). For a trait limited to primitive types,
/// see [`Prim`].
///
/// Note: For unit-aware types, the return types of operations like
/// multiplication, division, and square root must still be `Self`.
/// This is mathematically incorrect (e.g. `Meters * Meters` should result in
/// `MetersSquared`, and `Sqrt<MetersSquared>` should result in `Meters`),
/// but Rust's type system is not expressive enough to encode such behavior.
///
/// If you want to initialize a `Num` type with a specific numeric literal, use the [`num!`] macro.
///
/// Note that implementors of `Num` are not required to support common values like `0`, `1`, etc.
/// For example, a type might only represent values in the range `100..=356`. In that case,
/// trying to construct `0` or `1` using the [`num!`] macro would fail at compile time.
/// This allows `Num` to be used for restricted or domain-specific number types with full
/// compile-time validation of literal values.
pub trait Num:
    FromIntLiteral
    + AbsDiff<Output = Self>
    + MinMax
    + TruncRoot
    + Round
    + Signed<BoolMapped = bool>
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
{
}

impl Num for u8 {}
impl Num for u16 {}
impl Num for u32 {}
impl Num for u64 {}
impl Num for u128 {}
impl Num for usize {}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for i128 {}
impl Num for isize {}

impl Num for f32 {}
impl Num for f64 {}
