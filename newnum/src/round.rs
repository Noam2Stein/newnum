pub trait Round: Sized {
    fn round(self) -> Self;
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn atrunc(self) -> Self;
}

pub trait Whole: Round {}

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

impl Round for f32 {
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
impl Round for f64 {
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
