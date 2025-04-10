/// trait for ```min```, ```max``` fns.
///
/// Can be implemented by non ```PartialOrd``` number-containers types,
/// by returning the min/max of each element.
pub trait MinMax: Sized {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;

    fn clamp(self, min: Self, max: Self) -> Self {
        self.min(max).max(min)
    }
}

macro_rules! prim_impl {
    ($type:ident) => {
        impl MinMax for $type {
            fn min(self, other: Self) -> Self {
                if self < other {
                    self
                } else {
                    other
                }
            }
            fn max(self, other: Self) -> Self {
                if self > other {
                    self
                } else {
                    other
                }
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                if self < min {
                    min
                } else if self > max {
                    max
                } else {
                    self
                }
            }
        }
    };
}
prim_impl!(u8);
prim_impl!(u16);
prim_impl!(u32);
prim_impl!(u64);
prim_impl!(u128);
prim_impl!(usize);
prim_impl!(i8);
prim_impl!(i16);
prim_impl!(i32);
prim_impl!(i64);
prim_impl!(i128);
prim_impl!(isize);
prim_impl!(f32);
prim_impl!(f64);
