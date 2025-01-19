use crate::{Whole, WholeEquivalent};

/// Trait for the round API (round, floor...).
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
///
/// impl [```Whole```] for types that are always round (for example ```u8```),
/// which will auto impl ```Round``` because when being generic over a number type,
/// it makes sense to round a number even if it winds up to be an always round type.
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
/// where ```IRound``` returns ```Self::Output``` which represents the round equivalent of ```Self```.
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
///
/// impl [```Whole```] for types that are always round (for example ```u8```),
/// which will auto impl ```IRound``` because when being generic over a number type,
/// it makes sense to round a number even if it winds up to be an always round type.
pub trait IRound: Sized + WholeEquivalent {
    fn iround(self) -> Self::Whole;
    fn ifloor(self) -> Self::Whole;
    fn iceil(self) -> Self::Whole;
    fn itrunc(self) -> Self::Whole;
    fn iatrunc(self) -> Self::Whole;
}

impl<T: Whole> Round for T {
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

impl<T: Whole> IRound for T {
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
