pub trait Sign: Sized {
    type Bool;

    fn is_positive(&self) -> Self::Bool;
    fn is_negative(&self) -> Self::Bool;
    fn is_zero(&self) -> Self::Bool;
    fn signum(self) -> Self;
}

pub trait Zero: Sign {
    fn zero() -> Self;
}
pub trait Positive: Sign {
    fn abs(self) -> Self;
}
pub trait Negative: Sign {
    fn neg_abs(self) -> Self;
}

// negative traits - so when rust supports !Positive these will be removed
pub trait NotPositive: Sign {}
pub trait NotNegative: Sign {}
pub trait NotZero: Sign {}

// trait aliases - so when rust supports those these will be removed
pub trait OnlyPositive: NotNegative + NotZero + Positive {}
pub trait OnlyNegative: Negative + NotZero + NotPositive {}
pub trait OnlyZero: NotNegative + NotPositive + Zero {}
pub trait PositiveOrZero: NotNegative + Zero + Positive {}
pub trait NegativeOrZero: Negative + Zero + NotPositive {}
pub trait PositiveOrNegative: Negative + NotZero + Positive {}
pub trait FullySigned: Negative + Zero + Positive {}

impl<T: NotNegative + NotZero + Positive> OnlyPositive for T {}
impl<T: NotZero + NotPositive + Negative> OnlyNegative for T {}
impl<T: NotNegative + NotPositive + Zero> OnlyZero for T {}
impl<T: NotNegative + Zero + Positive> PositiveOrZero for T {}
impl<T: Negative + Zero + NotPositive> NegativeOrZero for T {}
impl<T: Negative + NotZero + Positive> PositiveOrNegative for T {}
impl<T: Negative + Zero + Positive> FullySigned for T {}

macro_rules! impl_sign_for_unsigned_ints {
    ($($type:ident)*) => {$(
        impl Sign for $type {
            type Bool = bool;

            #[inline(always)]
            fn is_positive(&self) -> Self::Bool {
                *self != 0
            }
            #[inline(always)]
            fn is_negative(&self) -> Self::Bool {
                false
            }
            #[inline(always)]
            fn is_zero(&self) -> Self::Bool {
                *self == 0
            }
            #[inline(always)]
            fn signum(self) -> Self {
                (self != 0) as $type
            }
        }
        impl Zero for $type {
            #[inline(always)]
            fn zero() -> Self {
                0
            }
        }
        impl Positive for $type {
            #[inline(always)]
            fn abs(self) -> Self {
                self
            }
        }
        impl NotNegative for $type {}
    )*};
}
impl_sign_for_unsigned_ints!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_sign_for_signed_ints {
    ($($type:ident)*) => {$(
        impl Sign for $type {
            type Bool = bool;

            #[inline(always)]
            fn is_positive(&self) -> Self::Bool {
                *self > 0
            }
            #[inline(always)]
            fn is_negative(&self) -> Self::Bool {
                *self < 0
            }
            #[inline(always)]
            fn is_zero(&self) -> Self::Bool {
                *self == 0
            }
            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }
        }
        impl Zero for $type {
            #[inline(always)]
            fn zero() -> Self {
                0
            }
        }
        impl Positive for $type {
            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }
        }
        impl Negative for $type {
            #[inline(always)]
            fn neg_abs(self) -> Self {
                -self.abs()
            }
        }
    )*};
}
impl_sign_for_signed_ints!(i8 i16 i32 i64 i128 isize);

macro_rules! impl_sign_for_floats {
    ($($type:ident)*) => {
        $(
            impl Sign for $type {
                type Bool = bool;

                #[inline(always)]
                fn is_positive(&self) -> Self::Bool {
                    self.is_sign_positive() && *self != 0.0
                }
                #[inline(always)]
                fn is_negative(&self) -> Self::Bool {
                    self.is_sign_negative() && *self != 0.0
                }
                #[inline(always)]
                fn is_zero(&self) -> Self::Bool {
                    *self == 0.0
                }
                #[inline(always)]
                fn signum(self) -> Self {
                    if self == 0.0 {
                        0.0
                    } else {
                        self.signum()
                    }
                }
            }
            impl Zero for $type {
                #[inline(always)]
                fn zero() -> Self {
                    0.0
                }
            }
            impl Positive for $type {
                #[inline(always)]
                fn abs(self) -> Self {
                    self.abs()
                }
            }
            impl Negative for $type {
                #[inline(always)]
                fn neg_abs(self) -> Self {
                    -self.abs()
                }
            }
        )*
    };
}
impl_sign_for_floats!(f32 f64);
