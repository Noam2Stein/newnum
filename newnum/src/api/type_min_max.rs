/// For types with a minimum value.
///
/// * Not all `Num`s implement `TypeMin` / `TypeMax`, because they may not have value limits.
/// For example a heap allocated int could rise infinitly until there is no more heap memory.
pub trait TypeMin {
    fn type_min() -> Self;
}

/// For types with a maximum value.
///
/// * Not all `Num`s implement `MinValue` / `MaxValue`, because they may not have value limits.
/// For example a heap allocated int could rise infinitly until there is no more heap memory.
pub trait TypeMax {
    fn type_max() -> Self;
}

/// For types without a minimum value.
///
/// * In the future when rust supports negative traits (`!SomeTrait`),
/// this trait will be replaced with `!TypeMin`.
pub trait NoTypeMin: PartialOrd {}

/// For types without a maximum value.
///
/// * In the future when rust supports negative traits (`!SomeTrait`),
/// this trait will be replaced with `!TypeMax`.
pub trait NoTypeMax: PartialOrd {}

/// Trait for types with both minimum and maximum values.
///
/// * This trait is a shortcut for `MinValue + MaxValue`, so it is auto implemented.
///
/// * In the future when rust supports trait aliases (like type aliases), this will be made into a trait alias.
pub trait Bounded: TypeMin + TypeMax {}

/// Trait for types with no minimum AND no maximum value.
///
/// * This trait is a shortcut for `NoMinValue + NoMaxValue`, so it is auto implemented.
///
/// * In the future when rust supports trait aliases (like type aliases), this will be made into a trait alias.
pub trait Unbounded: NoTypeMin + NoTypeMax {}

impl<T: TypeMin + TypeMax> Bounded for T {}
impl<T: NoTypeMin + NoTypeMax> Unbounded for T {}

macro_rules! prim_impl {
    ($ty:ty) => {
        impl TypeMin for $ty {
            fn type_min() -> Self {
                Self::MIN
            }
        }

        impl TypeMax for $ty {
            fn type_max() -> Self {
                Self::MAX
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
