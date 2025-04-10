mod util;

mod derive_empty;
mod derive_round;
mod derive_sign;
mod derive_trig;
mod derive_whole;
mod num;

//
//
//
// SIGN DERIVE MACROS
//
//
//

/// `Sign` derive macro.
/// `Sign` is a trait with methods that check sign attributes.
/// This macro expects the type to only have one field,
/// and redirects the trait methods to the field as `Sign`.
///
/// `Sign`'s methods return `Self::BoolMapped`.
/// This macro expects the field's `BoolMapped` type to be `bool`.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Sign` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Sign)]
/// #[derive_bound(Sign; T: Sign)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Sign, attributes(derive_bound))]
pub fn sign_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_sign::sign_derive_macro(input)
}

//
//
//
// ROUND DERIVE MACRO
//
//
//

/// `Round` derive macro.
/// for each method,
/// the derive implementation maps each field using the method.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Round` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Round)]
/// #[derive_bound(Round; T: Round)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Round, attributes(derive_bound))]
pub fn round_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_round::round_derive_macro(input)
}

/// `Whole` derive macro.
/// `Whole` is a trait that has no methods and is used to mark types as always round.
/// This derive macro doesn't verify anything about the data in the type,
/// so make sure the type should be `Whole` when using this derive macro.
///
/// This derive macro also derives `Round`,
/// with an implementation that doesn't do anything to round the values because they are already whole.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Whole` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// * If `derive_bound` is used, the `Round` trait will not be derived automatically,
///   and will need to be seperately derived using the `Round` derive macro.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Whole, Round)]
/// #[derive_bound(Whole; T: Whole)]
/// #[derive_bound(Round; T: Round)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Whole, attributes(derive_bound))]
pub fn whole_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_whole::whole_derive_macro(input)
}

//
//
//
// TRIG DERIVE MACROS
//
//
//

/// `Trig` derive macro.
/// for each method,
/// the derive implementation maps each field using the method.
///
/// `Trig`'s methods return `Self::Output`.
/// This macro expects that for each field: `T::Output = T`.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Trig` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Trig)]
/// #[derive_bound(Trig; T: Trig)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Trig, attributes(derive_bound))]
pub fn trig_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_trig::trig_derive_macro(input)
}

/// `ATrig` derive macro.
/// for each method,
/// the derive implementation maps each field using the method.
///
/// `ATrig`'s methods return `Self::Output`.
/// This macro expects that for each field: `T::Output = T`.
///
/// ### Generics
///
/// For types with generic parameters,
/// `ATrig` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(ATrig)]
/// #[derive_bound(ATrig; T: ATrig)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(ATrig, attributes(derive_bound))]
pub fn atrig_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_trig::atrig_derive_macro(input)
}

/// `Hyper` derive macro.
/// for each method,
/// the derive implementation maps each field using the method.
///
/// `Hyper`'s methods return `Self::Output`.
/// This macro expects that for each field: `T::Output = T`.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Hyper` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Hyper)]
/// #[derive_bound(Hyper; T: Hyper)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Hyper, attributes(derive_bound))]
pub fn hyper_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_trig::hyper_derive_macro(input)
}

/// `AHyper` derive macro.
/// for each method,
/// the derive implementation maps each field using the method.
///
/// `AHyper`'s methods return `Self::Output`.
/// This macro expects that for each field: `T::Output = T`.
///
/// ### Generics
///
/// For types with generic parameters,
/// `AHyper` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(AHyper)]
/// #[derive_bound(AHyper; T: AHyper)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(AHyper, attributes(derive_bound))]
pub fn ahyper_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_trig::ahyper_derive_macro(input)
}

//
//
//
// EMPTY DERIVE MACROS
//
//
//

/// `Num` derive macro.
/// `Num` is an empty trait that has many super-traits,
/// this macro only derives `Num` and not its super-traits.
///
/// ### Generics
///
/// For types with generic parameters,
/// `Num` will be implemented with no additional trait-bounds.
///
/// To add bounds to the derive, use the `derive_bound` attribute which follows this syntax:
/// `#[derive_bound(<trait-ident>; <where-predicate>, ...)]`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// #[derive(Num)]
/// #[derive_bound(Num; T: Num)]
/// struct Fun<T>(T);
/// ```
#[proc_macro_derive(Num, attributes(derive_bound))]
pub fn num_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_empty::num_derive_macro(input)
}

//
//
//
// NUM MACROS
//
//
//

/// Converts a numeric literal into a `Num` type, generating a compile-time error if the literal is out of range for the type.
///
/// * the type doesn't have to implement `Num`, but it must implement `FromIntLiteral` and `FromFloatLiteral` for float literal support.
///
/// ### Syntax
///
/// `num!(<literal>)` or `num!(<literal>: <type>)`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// fn inc(value: &mut impl Num) {
///    *value += num!(1)
/// }
/// ```
///
/// ### Compile-Time Error
///
/// Because of rust const-fn limitations,
/// the error shown when the literal is out of range is cryptic and is shown at the macro call-site.
///
/// Example:
///
///```
/// use newnum::*;
///
/// fn add_alot(value: &mut impl Num) {
///     //        |<- ERROR: evaluation of `add_alot::num_macro_fn::<u8>::{constant#0}` failed
///     //        |
///     *value += num!(1000)
/// }
///
/// fn main() {
///     let mut i = 5u8;
///     add_alot(&mut i);
/// }
/// ```
#[proc_macro]
pub fn num(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num::num(input)
}

/// Converts a numeric literal into a `Num` type, generating a compile-time error if the literal is out of range for the type.
/// In comparison to `num!`, this macro also accepts literals that can only be approximately represented by the type.
///
/// * the type doesn't have to implement `Num`, but it must implement `FromIntLiteral` and `FromFloatLiteral` for float literal support.
///
/// ### Syntax
///
/// `num_approx!(<literal>)` or `num_approx!(<literal>: <type>)`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// fn example<T: Float>() -> T {
///     num_approx!(16_777_217)
/// }
/// ```
///
/// ### Compile-Time Error
///
/// Because of rust const-fn limitations,
/// the error shown when the literal is out of range is cryptic and is shown at the macro call-site.
///
/// Example:
///
///```
/// use newnum::*;
///
/// fn add_alot(value: &mut impl Num) {
///     //        |<- ERROR: evaluation of `add_alot::num_macro_fn::<u8>::{constant#0}` failed
///     //        |
///     *value += num_approx!(1000)
/// }
///
/// fn main() {
///     let mut i = 5u8;
///     add_alot(&mut i);
/// }
/// ```
#[proc_macro]
pub fn num_approx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num::num_approx(input)
}

/// Is `num!` but uses `crate` as the crate path. This is useful for macros that are used in the `newnum` crate itself.
///
/// Converts a numeric literal into a `Num` type, generating a compile-time error if the literal is out of range for the type.
///
/// * the type doesn't have to implement `Num`, but it must implement `FromIntLiteral` and `FromFloatLiteral` for float literal support.
///
/// ### Syntax
///
/// `num!(<literal>)` or `num!(<literal>: <type>)`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// fn inc(value: &mut impl Num) {
///    *value += num!(1)
/// }
/// ```
///
/// ### Compile-Time Error
///
/// Because of rust const-fn limitations,
/// the error shown when the literal is out of range is cryptic and is shown at the macro call-site.
///
/// Example:
///
///```
/// use newnum::*;
///
/// fn add_alot(value: &mut impl Num) {
///     //        |<- ERROR: evaluation of `add_alot::num_macro_fn::<u8>::{constant#0}` failed
///     //        |
///     *value += num!(1000)
/// }
///
/// fn main() {
///     let mut i = 5u8;
///     add_alot(&mut i);
/// }
/// ```
#[proc_macro]
pub fn internal_num(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num::internal_num(input)
}

/// Is `num_approx!` but uses `crate` as the crate path. This is useful for macros that are used in the `newnum` crate itself.
///
/// Converts a numeric literal into a `Num` type, generating a compile-time error if the literal is out of range for the type.
/// In comparison to `num!`, this macro also accepts literals that can only be approximately represented by the type.
///
/// * the type doesn't have to implement `Num`, but it must implement `FromIntLiteral` and `FromFloatLiteral` for float literal support.
///
/// ### Syntax
///
/// `num_approx!(<literal>)` or `num_approx!(<literal>: <type>)`.
///
/// ### Example
///
/// ```
/// use newnum::*;
///
/// fn example<T: Float>() -> T {
///     num_approx!(16_777_217)
/// }
/// ```
///
/// ### Compile-Time Error
///
/// Because of rust const-fn limitations,
/// the error shown when the literal is out of range is cryptic and is shown at the macro call-site.
///
/// Example:
///
///```
/// use newnum::*;
///
/// fn add_alot(value: &mut impl Num) {
///     //        |<- ERROR: evaluation of `add_alot::num_macro_fn::<u8>::{constant#0}` failed
///     //        |
///     *value += num_approx!(1000)
/// }
///
/// fn main() {
///     let mut i = 5u8;
///     add_alot(&mut i);
/// }
/// ```
#[proc_macro]
pub fn internal_num_approx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num::internal_num_approx(input)
}
