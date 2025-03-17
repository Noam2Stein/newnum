/// Trait for types that have a mathamatical sign (maybe a number or a number container).
///
/// doesn't require the type to represent positive / negative / zero values,
/// and so doesn't have functions like ```abs``` because if the type can't represent positive values but only negative ones,
/// it can't return a positive value out of ```abs```.
pub trait Sign {
    /// The bool equivalent of ```Self```.
    ///
    /// For a number this will always be ```bool```,
    /// but for a number container this can be the container mapped to ```bool```
    /// (for example ```<Vec2<T> as Sign>::BoolMapped = Vec2<bool>```)
    type BoolMapped;

    /// Returns ```true``` if ```self``` is larger than zero.
    ///
    /// This means that even for floats who store a sign thats either positive or negative even when the number is equal to zero,
    /// this function returns ```false``` for ```self == 0```.
    ///
    /// For a sign that is either positive or negative (so may return true for zero) use ```is_sign_positive```.
    fn is_positive(&self) -> Self::BoolMapped;

    /// Returns ```true``` if ```self``` is smaller than zero.
    ///
    /// This means that even for floats who store a sign thats either positive or negative even when the number is equal to zero,
    /// this function returns ```false``` for ```self == 0```.
    ///
    /// For a sign that is either positive or negative (so may return true for zero) use ```is_sign_negative```.
    fn is_negative(&self) -> Self::BoolMapped;

    fn is_zero(&self) -> Self::BoolMapped;

    /// Returns ```true``` if ```self```'s sign is positive.
    ///
    /// If ```self``` is equal to zero:
    /// * If ```Self``` stores the number's sign even in a zero state (```+0``` / ```-0```) the sign is used.
    /// * If ```Self``` doesn't store the number's sign in a zero state, the number is considered positive.
    fn is_sign_positive(&self) -> Self::BoolMapped;

    /// Returns ```true``` if ```self```'s sign is negative.
    ///
    /// If ```self``` is equal to zero:
    /// * If ```Self``` stores the number's sign even in a zero state (```+0``` / ```-0```) the sign is used.
    /// * If ```Self``` doesn't store the number's sign in a zero state, the number is considered positive.
    fn is_sign_negative(&self) -> Self::BoolMapped;
}

/// Trait for types that can represent positive values (maybe a number or a number container).
/// Valid even if only one value is representable as long as its larger than zero.
///
/// * Doesn't mean the type can only represent positive values or will always be positive.
/// For that use ```OnlyPositive```.
pub trait Positive: Sign {
    fn abs(self) -> Self;
}
/// Trait for types that can represent negative values (maybe a number or a number container).
/// Valid even if only one value is representable as long as its smaller than zero.
///
/// * Doesn't mean the type can only represent negative values or will always be negative.
/// For that use ```OnlyNegative```.
pub trait Negative: Sign {
    fn neg_abs(self) -> Self;
}
/// Trait for types that can represent zero (maybe a number or a number container).
pub trait Zero: Sign {
    fn zero() -> Self;
}

/// Trait for types that can't represent positive values (maybe a number or a number container).
/// Its always guarenteed that a value of the type is not positive (```<= 0```).
///
/// * In the future when rust supports negative traits (```!SomeTrait```),
/// this trait will be replaced with ```!Positive```.
pub trait NotPositive: Sign {}
/// Trait for types that can't represent negative values (maybe a number or a number container).
/// Its always guarenteed that a value of the type is not negative (```>= 0```).
///
/// * In the future when rust supports negative traits (```!SomeTrait```),
/// this trait will be replaced with ```!Negative```.
pub trait NotNegative: Sign {}
/// Trait for types that can't represent zero (maybe a number or a number container).
/// Its always guarenteed that a value of the type is not zero (```!= 0```).
///
/// * In the future when rust supports negative traits (```!SomeTrait```),
/// this trait will be replaced with ```!Zero```.
pub trait NotZero: Sign {}

/// Trait for types that can only represent positive values (maybe a number or a number container).
/// Its always guarenteed that a value of the type is positive (```> 0```).
///
/// * This trait is a shortcut for ```Positive + NotNegative + NotZero```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait OnlyPositive: NotNegative + NotZero + Positive {}
/// Trait for types that can only represent negative values (maybe a number or a number container).
/// Its always guarenteed that a value of the type is negative (```< 0```).
///
/// * This trait is a shortcut for ```Negative + NotPositive + NotZero```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait OnlyNegative: Negative + NotZero + NotPositive {}
/// Trait for types that can only represent the value zero (maybe a number or a number container).
/// Its always guarenteed that a value of the type is zero (```< 0```).
///
/// Can be useful for zero-sized types.
///
/// * This trait is a shortcut for ```Zero + NotPositive + NotNegative```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait OnlyZero: NotNegative + NotPositive + Zero {}
/// Trait for types that can only represent either positive values or zero, so no negative numbers (maybe a number or a number container).
/// Its always guarenteed that a value of the type is either positive or zero (```>= 0```).
///
/// This is what many call "unsigned number types" (```u8```, ```usize```... and not ```i32```, ```f32```).
///
/// * This trait is a shortcut for ```Positive + Zero + NotNegative```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait PositiveOrZero: NotNegative + Zero + Positive {}
/// Trait for types that can only represent either negative values or zero, so no positive numbers (maybe a number or a number container).
/// Its always guarenteed that a value of the type is either negative or zero (```<= 0```).
///
/// * This trait is a shortcut for ```Negative + Zero + NotPositive```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait NegativeOrZero: Negative + Zero + NotPositive {}
/// Trait for types that can only represent either positive values or negative values, so no zero (maybe a number or a number container).
/// Its always guarenteed that a value of the type is not zero (```!= 0```).
///
/// * This trait is a shortcut for ```Positive + Negative + NotZero```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait PositiveOrNegative: Negative + NotZero + Positive {}
/// Trait for types that can represent both positive values, negative values and zero (maybe a number or a number container).
/// This is what many call "signed number types" (```i32```, ```f64```... and not ```u8```, ```usize```...).
///
/// * This trait is a shortcut for ```Positive + Negative + Zero```, so it is auto implemented.
///
/// * In the future when rust will support trait aliases (like type aliases), this will be made a trait alias.
pub trait FullySigned: Negative + Zero + Positive {}

impl<T: NotNegative + NotZero + Positive> OnlyPositive for T {}
impl<T: NotZero + NotPositive + Negative> OnlyNegative for T {}
impl<T: NotNegative + NotPositive + Zero> OnlyZero for T {}
impl<T: NotNegative + Zero + Positive> PositiveOrZero for T {}
impl<T: Negative + Zero + NotPositive> NegativeOrZero for T {}
impl<T: Negative + NotZero + Positive> PositiveOrNegative for T {}
impl<T: Negative + Zero + Positive> FullySigned for T {}

pub trait Signum: Sign {
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
        impl Sign for $type {
            type BoolMapped = bool;

            #[inline(always)]
            fn is_positive(&self) -> Self::BoolMapped {
                *self != 0
            }

            #[inline(always)]
            fn is_negative(&self) -> Self::BoolMapped {
                false
            }

            #[inline(always)]
            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0
            }

            #[inline(always)]
            fn is_sign_positive(&self) -> Self::BoolMapped {
                true
            }

            #[inline(always)]
            fn is_sign_negative(&self) -> Self::BoolMapped {
                false
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

        impl Signum for $type {
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
        impl Sign for $type {
            type BoolMapped = bool;

            #[inline(always)]
            fn is_positive(&self) -> Self::BoolMapped {
                *self > 0
            }

            #[inline(always)]
            fn is_negative(&self) -> Self::BoolMapped {
                *self < 0
            }

            #[inline(always)]
            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0
            }

            #[inline(always)]
            fn is_sign_positive(&self) -> Self::BoolMapped {
                *self >= 0
            }

            #[inline(always)]
            fn is_sign_negative(&self) -> Self::BoolMapped {
                *self < 0
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

        impl Signum for $type {
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
        impl Sign for $type {
            type BoolMapped = bool;

            #[inline(always)]
            fn is_positive(&self) -> Self::BoolMapped {
                self.is_sign_positive() && *self != 0.0
            }

            #[inline(always)]
            fn is_negative(&self) -> Self::BoolMapped {
                self.is_sign_negative() && *self != 0.0
            }

            #[inline(always)]
            fn is_zero(&self) -> Self::BoolMapped {
                *self == 0.0
            }

            #[inline(always)]
            fn is_sign_positive(&self) -> Self::BoolMapped {
                $type::is_sign_positive(*self)
            }

            #[inline(always)]
            fn is_sign_negative(&self) -> Self::BoolMapped {
                $type::is_sign_negative(*self)
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

        impl Signum for $type {
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
