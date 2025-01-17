/// Trait for the round API (round, floor...).
///
/// Doesn't require ```Self``` to be a number because non-number types can benefit from this too.
/// For example number containers like vectors or matricies.
///
/// impl [```Whole```] for types that are always round (for example ```u8```),
/// which will auto impl ```Round``` because when being generic over a number type,
/// it makes sense to round a number even if it winds up to be an always round type.
pub trait Round: Sized {
    type Output;

    fn round(self) -> Self::Output;
    fn floor(self) -> Self::Output;
    fn ceil(self) -> Self::Output;
    fn trunc(self) -> Self::Output;
    fn atrunc(self) -> Self::Output;
}

/// Trait for numbers or number containers that are always round (for example ```i32```).
/// Auto impls ```Round``` (rounding a whole value does nothing).
pub trait Whole {}

impl<T: Whole> Round for T {
    type Output = Self;

    #[inline(always)]
    fn round(self) -> Self::Output {
        self
    }
    #[inline(always)]
    fn floor(self) -> Self::Output {
        self
    }
    #[inline(always)]
    fn ceil(self) -> Self::Output {
        self
    }
    #[inline(always)]
    fn trunc(self) -> Self::Output {
        self
    }
    #[inline(always)]
    fn atrunc(self) -> Self::Output {
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

macro_rules! impl_round_for_floats {
    ($($type:ty)*) => {
        $(
            impl Round for $type {
                type Output = Self;

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
        )*
    };
}
impl_round_for_floats!(f32 f64);
