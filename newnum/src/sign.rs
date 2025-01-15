pub trait MaybeSigned: Sized {
    fn is_positive(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn signum(self) -> Self;
}

pub trait Zero: MaybeSigned {
    const ZERO: Self;
}
pub trait Positive: MaybeSigned {
    fn abs(self) -> Self;
}
pub trait Negative: MaybeSigned {
    fn neg_abs(self) -> Self;
}

// negative traits - so when rust supports !Positive these will be removed
pub trait NotPositive: MaybeSigned {}
pub trait NotNegative: MaybeSigned {}
pub trait NotZero: MaybeSigned {}

// trait aliases - so when rust supports those these will be removed
pub trait OnlyPositive: NotNegative + NotZero + Positive {}
pub trait OnlyNegative: Negative + NotZero + NotPositive {}
pub trait OnlyZero: NotNegative + NotPositive + Zero {}
pub trait PositiveOrZero: NotNegative + Zero + Positive {}
pub trait NegativeOrZero: Negative + Zero + NotPositive {}
pub trait PositiveOrNegative: Negative + NotZero + Positive {}
pub trait Signed: Negative + Zero + Positive {}

impl<T: NotNegative + NotZero + Positive> OnlyPositive for T {}
impl<T: NotZero + NotPositive + Negative> OnlyNegative for T {}
impl<T: NotNegative + NotPositive + Zero> OnlyZero for T {}
impl<T: NotNegative + Zero + Positive> PositiveOrZero for T {}
impl<T: Negative + Zero + NotPositive> NegativeOrZero for T {}
impl<T: Negative + NotZero + Positive> PositiveOrNegative for T {}
impl<T: Negative + Zero + Positive> Signed for T {}

macro_rules! impl_sign_for_unsigned_ints {
    ($($type:ident)*) => {$(
        impl MaybeSigned for $type {
            #[inline(always)]
            fn is_positive(&self) -> bool {
                *self != 0
            }
            #[inline(always)]
            fn is_negative(&self) -> bool {
                false
            }
            #[inline(always)]
            fn signum(self) -> Self {
                (self != 0) as $type
            }
        }
        impl Zero for $type {
            const ZERO: Self = 0;
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
        impl MaybeSigned for $type {
            #[inline(always)]
            fn is_positive(&self) -> bool {
                *self > 0
            }
            #[inline(always)]
            fn is_negative(&self) -> bool {
                *self < 0
            }
            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }
        }
        impl Zero for $type {
            const ZERO: Self = 0;
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

impl MaybeSigned for f32 {
    #[inline(always)]
    fn is_positive(&self) -> bool {
        self.is_sign_positive() && *self != 0.0
    }
    #[inline(always)]
    fn is_negative(&self) -> bool {
        self.is_sign_negative() && *self != 0.0
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
impl MaybeSigned for f64 {
    #[inline(always)]
    fn is_positive(&self) -> bool {
        self.is_sign_positive() && *self != 0.0
    }
    #[inline(always)]
    fn is_negative(&self) -> bool {
        self.is_sign_negative() && *self != 0.0
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

impl Zero for f32 {
    const ZERO: Self = 0.0;
}
impl Zero for f64 {
    const ZERO: Self = 0.0;
}

impl Positive for f32 {
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl Positive for f64 {
    #[inline(always)]
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Negative for f32 {
    #[inline(always)]
    fn neg_abs(self) -> Self {
        -self.abs()
    }
}
impl Negative for f64 {
    #[inline(always)]
    fn neg_abs(self) -> Self {
        -self.abs()
    }
}
