/// Trait for `trunc_sqrt` and `trunc_cbrt` methods, which compute the truncated square root and cube root of a number.
///
/// For non-numbers type (number-containers like `Vec2`) the logic of `sqrt` / `cbrt` should follow the logic of `Mul`.
/// For example, if `<Vec2 as Mul>` multiplies each component seperately, it should also `sqrt` each component seperately.
pub trait TruncRoot {
    fn trunc_sqrt(self) -> Self;
    fn trunc_cbrt(self) -> Self;
}

pub trait Root: TruncRoot {
    fn sqrt(self) -> Self;
    fn cbrt(self) -> Self;
}

macro_rules! int_impl {
    ($type:ident) => {
        impl TruncRoot for $type {
            fn trunc_sqrt(self) -> Self {
                const FLOAT_CONVERT_LIMIT: Option<$type> = (1 as $type).checked_shl(52);

                if let Some(convert_lim) = FLOAT_CONVERT_LIMIT {
                    if self < convert_lim {
                        (self as f64).sqrt() as Self
                    } else {
                        let max_sqrt = (0 as $type..)
                            .filter(|x| x.checked_mul(*x).is_some())
                            .last()
                            .unwrap();

                        let mut low = 1;
                        let mut high = (self / 2).min(max_sqrt);

                        while low <= high {
                            let mid = (low + high) / 2;
                            let mid_squared = mid * mid;
                            if mid_squared == self {
                                return mid;
                            } else if mid_squared < self {
                                low = mid + 1;
                            } else {
                                high = mid - 1;
                            }
                        }

                        high
                    }
                } else {
                    (self as f64).sqrt() as Self
                }
            }

            fn trunc_cbrt(self) -> Self {
                const FLOAT_CONVERT_LIMIT: Option<$type> = (1 as $type).checked_shl(52);

                if let Some(convert_lim) = FLOAT_CONVERT_LIMIT {
                    if self < convert_lim {
                        (self as f64).cbrt() as Self
                    } else {
                        let max_cbrt = (0 as $type..)
                            .filter(|x| {
                                x.checked_mul(*x)
                                    .map_or(None, |x| x.checked_mul(x))
                                    .is_some()
                            })
                            .last()
                            .unwrap();

                        let mut low = 1;
                        let mut high = (self / 2).min(max_cbrt);

                        while low <= high {
                            let mid = (low + high) / 2;
                            let mid_cubed = mid * mid * mid;
                            if mid_cubed == self {
                                return mid;
                            } else if mid_cubed < self {
                                low = mid + 1;
                            } else {
                                high = mid - 1;
                            }
                        }

                        high
                    }
                } else {
                    (self as f64).cbrt() as Self
                }
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
        impl TruncRoot for $type {
            fn trunc_sqrt(self) -> Self {
                self.sqrt().trunc()
            }
            fn trunc_cbrt(self) -> Self {
                self.cbrt().trunc()
            }
        }
        impl Root for $type {
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            fn cbrt(self) -> Self {
                self.cbrt()
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
