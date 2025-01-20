use crate::WholeEquivalent;

/// Trait for the round API (round, floor...).
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
pub trait Round: Sized {
    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn atrunc(self) -> Self;
}

/// Trait for a round-to-int API (iround, ifloor...).
///
/// ```Round``` returns ```Self```,
/// where ```IRound``` returns ```Self::Whole``` which represents the whole equivalent of ```Self```.
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
pub trait IRound: Sized + WholeEquivalent {
    fn iround(self) -> Self::Whole;
    fn ifloor(self) -> Self::Whole;
    fn iceil(self) -> Self::Whole;
    fn itrunc(self) -> Self::Whole;
    fn iatrunc(self) -> Self::Whole;
}

macro_rules! int_impl {
    ($type:ident) => {
        impl Round for $type {
            #[inline(always)]
            fn round(self) -> Self {
                self
            }

            #[inline(always)]
            fn floor(self) -> Self {
                self
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                self
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                self
            }

            #[inline(always)]
            fn atrunc(self) -> Self {
                self
            }
        }

        impl IRound for $type {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iatrunc(self) -> Self::Whole {
                self
            }
        }
    };
}
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);

macro_rules! float_impl {
    ($type:ident) => {
        impl Round for $type {
            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }
            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }
            #[inline(always)]
            fn trunc(self) -> Self {
                self.trunc()
            }
            #[inline(always)]
            fn atrunc(self) -> Self {
                if self.is_sign_positive() {
                    self.ceil()
                } else {
                    self.floor()
                }
            }
        }

        impl IRound for $type {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self.round() as _
            }
            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self.floor() as _
            }
            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self.ceil() as _
            }
            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self.trunc() as _
            }
            #[inline(always)]
            fn iatrunc(self) -> Self::Whole {
                self.atrunc() as _
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
