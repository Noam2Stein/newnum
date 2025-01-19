/// Trait for numbers or number containers that are always round (for example ```i32```).
/// Auto impls ```Round``` and ```IRound``` (rounding a whole value does nothing).
pub trait Whole {}

pub trait WholeEquivalent {
    type Whole: Whole;

    fn whole(self) -> Self::Whole;
}

impl<T: Whole> WholeEquivalent for T {
    type Whole = Self;

    fn whole(self) -> Self::Whole {
        self
    }
}

impl Whole for u8 {}
impl Whole for u16 {}
impl Whole for u32 {}
impl Whole for u64 {}
impl Whole for u128 {}
impl Whole for usize {}

impl Whole for i8 {}
impl Whole for i16 {}
impl Whole for i32 {}
impl Whole for i64 {}
impl Whole for i128 {}
impl Whole for isize {}

macro_rules! float_impl {
    ($type:ident => $whole:ident) => {
        impl WholeEquivalent for $type {
            type Whole = $whole;

            #[inline(always)]
            fn whole(self) -> Self::Whole {
                self as _
            }
        }
    };
}
float_impl!(f32 => i32);
float_impl!(f64 => i64);
