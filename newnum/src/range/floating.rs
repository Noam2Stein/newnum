pub trait FloatingEquivalent {
    type Floating: Floating;

    fn floating(self) -> Self::Floating;
}

pub trait Floating: FloatingEquivalent<Floating = Self> {}

impl<T: FloatingEquivalent<Floating = Self>> Floating for T {}

macro_rules! float_impl {
    ($type:ident) => {
        impl FloatingEquivalent for $type {
            type Floating = Self;

            #[inline(always)]
            fn floating(self) -> Self::Floating {
                self
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);

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
int_impl!(i8 => f32);
int_impl!(i16 => f32);
int_impl!(i32 => f64);
