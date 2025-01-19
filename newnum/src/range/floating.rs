pub trait Floating {}

pub trait FloatingEquivalent {
    type Floating: Floating;

    fn floating(self) -> Self::Floating;
}

impl<T: Floating> FloatingEquivalent for T {
    type Floating = Self;

    #[inline(always)]
    fn floating(self) -> Self::Floating {
        self
    }
}

impl Floating for f32 {}
impl Floating for f64 {}

macro_rules! int_impl {
    ($type:ident => $floating:ident) => {
        impl FloatingEquivalent for $type {
            type Floating = $floating;

            #[inline(always)]
            fn floating(self) -> Self::Floating {
                self as _
            }
        }
    };
}
int_impl!(u8 => f32);
int_impl!(u16 => f32);
int_impl!(u32 => f64);
int_impl!(u64 => f64);
int_impl!(u128 => f64);
int_impl!(usize => f64);
int_impl!(i8 => f32);
int_impl!(i16 => f32);
int_impl!(i32 => f64);
int_impl!(i64 => f64);
int_impl!(i128 => f64);
int_impl!(isize => f64);
