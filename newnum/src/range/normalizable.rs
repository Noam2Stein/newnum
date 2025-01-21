use super::{Sign, Zero};

pub trait Normalizable: Sign {
    /// Returns either ```1``` or ```-1``` based on the number's sign:
    /// * positive ```=> 1```,
    /// * negative ```=> -1```.
    ///
    /// If ```self``` is equal to zero:
    /// * If ```Self``` stores the number's sign even in a zero state (```+0``` / ```-0```) the sign is used.
    /// * If ```Self``` doesn't store the number's sign in a zero state, the number is considered positive.
    fn signumf(self) -> Self;

    /// Returns either ```1```, ```-1``` or ```0``` based on the number's sign:
    /// * ```self > 0 => 1```,
    /// * ```self < 0 => -1```,
    /// * ```self == 0 => 0```.
    ///
    /// This means that even for floats who store a sign thats either positive or negative even when the number is equal to zero,
    /// this function returns zero for ```self == 0```.
    ///
    /// For a signum that is either ```1``` or ```-1``` use ```signumf```.
    fn signum(self) -> Self
    where
        Self: Zero;
}

macro_rules! uint_impl {
    ($type:ident) => {
        impl Normalizable for $type {
            #[inline(always)]
            fn signum(self) -> Self {
                (self != 0) as $type
            }

            #[inline(always)]
            fn signumf(self) -> Self {
                1
            }
        }
    };
}
uint_impl!(u8);
uint_impl!(u16);
uint_impl!(u32);
uint_impl!(u64);
uint_impl!(u128);
uint_impl!(usize);

macro_rules! sint_impl {
    ($type:ident) => {
        impl Normalizable for $type {
            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }

            #[inline(always)]
            fn signumf(self) -> Self {
                if self.is_sign_positive() {
                    1
                } else {
                    -1
                }
            }
        }
    };
}
sint_impl!(i8);
sint_impl!(i16);
sint_impl!(i32);
sint_impl!(i64);
sint_impl!(i128);
sint_impl!(isize);

macro_rules! float_impl {
    ($type:ident) => {
        impl Normalizable for $type {
            #[inline(always)]
            fn signum(self) -> Self {
                if self == 0.0 {
                    0.0
                } else {
                    self.signum()
                }
            }

            #[inline(always)]
            fn signumf(self) -> Self {
                $type::signum(self)
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);
