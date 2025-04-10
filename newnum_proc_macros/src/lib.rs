mod derive_attributes;
mod derive_empty;
mod derive_sign;
mod num;

//
//
//
// SIGN DERIVE MACROS
//
//
//

#[proc_macro_derive(Sign, attributes(derive_bound))]
pub fn sign_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_sign::sign_derive_macro(input)
}

//
//
//
// EMPTY DERIVE MACROS
//
//
//

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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
/// use newnum::num;
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
