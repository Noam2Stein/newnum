use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Lit, Token};

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
    num_macro_helper(
        "from_int_literal",
        "from_float_literal",
        quote! { ::newnum },
        input,
    )
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
    num_macro_helper(
        "approx_from_int_literal",
        "approx_from_float_literal",
        quote! { ::newnum },
        input,
    )
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
    num_macro_helper(
        "from_int_literal",
        "from_float_literal",
        quote! { crate },
        input,
    )
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
    num_macro_helper(
        "approx_from_int_literal",
        "approx_from_float_literal",
        quote! { crate },
        input,
    )
}

fn num_macro_helper(
    int_fn_ident: &str,
    float_fn_ident: &str,
    crate_path: TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        literal: Lit,
        #[prefix(Option<Token![:]> as punct)]
        #[parse_if(punct.is_some())]
        ty: Option<TokenStream>,
    }

    let Input { literal, ty } = parse_macro_input!(input as Input);

    let (literal_ty, from_trait, from_fn) = if let Lit::Float(_) = literal {
        (quote! { f64 }, quote! { FromFloatLiteral }, float_fn_ident)
    } else {
        (quote! { u128 }, quote! { FromIntLiteral }, int_fn_ident)
    };

    let from_fn = format_ident!("{from_fn}");
    let ty = ty.unwrap_or_else(|| quote! { _});

    quote! {
        {
            const MACRO_INPUT: #literal_ty = #literal;

            {
                fn num_macro_fn<NumMacroType: #crate_path::#from_trait>() -> NumMacroType {
                    if const {
                        if MACRO_INPUT < <NumMacroType as #crate_path::FromIntLiteral>::MIN_LITERAL as #literal_ty {
                            panic!("literal out of range")
                        }

                        if MACRO_INPUT > <NumMacroType as #crate_path::FromIntLiteral>::MAX_LITERAL as #literal_ty {
                            panic!("literal out of range")
                        }

                        true
                    } {
                        unsafe { <NumMacroType as #crate_path::#from_trait>::#from_fn(MACRO_INPUT) }
                    } else {
                        unreachable!()
                    }
                }

                num_macro_fn::<#ty>()
            }
        }
    }
    .into()
}
