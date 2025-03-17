use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error, Expr, ExprCast};

#[proc_macro]
pub fn int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let (expr, fn_call) = if let Expr::Cast(ExprCast {
        attrs: _,
        expr,
        as_token: _,
        ty,
    }) = input
    {
        fn verify_expr(expr: &Expr) -> Result<(), proc_macro::TokenStream> {
            match expr {
                Expr::Lit(_) => Ok(()),
                Expr::Unary(expr) => verify_expr(&expr.expr),
                _ => Err(Error::new(expr.span(), "`( ... ) required`")
                    .to_compile_error()
                    .into()),
            }
        }

        if let Err(error) = verify_expr(&expr) {
            return error;
        }

        (
            *expr,
            quote! {
                num_macro_fn::<#ty>()
            },
        )
    } else {
        (
            input,
            quote! {
                num_macro_fn()
            },
        )
    };

    quote! {
        {
            const MACRO_INPUT: i128 = #expr;

            {
                fn num_macro_fn<NumMacroType: ::newnum::Num>() -> NumMacroType {
                    if const {
                        if let Some(min) = NumMacroType::MIN_CONSTANT {
                            if MACRO_INPUT < min {
                                panic!("number too small for given type")
                            }
                        }

                        if let Some(max) = NumMacroType::MAX_CONSTANT {
                            if MACRO_INPUT > max {
                                panic!("number too large for given type")
                            }
                        }

                        true
                    } {
                        unsafe { NumMacroType::int_constant(MACRO_INPUT) }
                    } else {
                        unreachable!()
                    }
                }

                #fn_call
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn float(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let (expr, fn_call) = if let Expr::Cast(ExprCast {
        attrs: _,
        expr,
        as_token: _,
        ty,
    }) = input
    {
        fn verify_expr(expr: &Expr) -> Result<(), proc_macro::TokenStream> {
            match expr {
                Expr::Lit(_) => Ok(()),
                Expr::Unary(expr) => verify_expr(&expr.expr),
                _ => Err(Error::new(expr.span(), "`( ... ) required`")
                    .to_compile_error()
                    .into()),
            }
        }

        if let Err(error) = verify_expr(&expr) {
            return error;
        }

        (
            *expr,
            quote! {
                num_macro_fn::<#ty>()
            },
        )
    } else {
        (
            input,
            quote! {
                num_macro_fn()
            },
        )
    };

    quote! {
        {
            const MACRO_INPUT: f64 = #expr;

            {
                fn num_macro_fn<NumMacroType: ::newnum::Num>() -> NumMacroType {
                    if const {
                        if !NumMacroType::ALLOW_FLOAT_CONSTANTS {
                            panic!("float constants not allowed for given type")
                        }

                        if let Some(min) = NumMacroType::MIN_CONSTANT {
                            if MACRO_INPUT < min as f64 {
                                panic!("number too small for given type")
                            }
                        }

                        if let Some(max) = NumMacroType::MAX_CONSTANT {
                            if MACRO_INPUT > max as f64 {
                                panic!("number too large for given type")
                            }
                        }

                        true
                    } {
                        unsafe { NumMacroType::float_constant(MACRO_INPUT) }
                    } else {
                        unreachable!()
                    }
                }

                #fn_call
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn crate_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let (expr, fn_call) = if let Expr::Cast(ExprCast {
        attrs: _,
        expr,
        as_token: _,
        ty,
    }) = input
    {
        fn verify_expr(expr: &Expr) -> Result<(), proc_macro::TokenStream> {
            match expr {
                Expr::Lit(_) => Ok(()),
                Expr::Unary(expr) => verify_expr(&expr.expr),
                _ => Err(Error::new(expr.span(), "`( ... ) required`")
                    .to_compile_error()
                    .into()),
            }
        }

        if let Err(error) = verify_expr(&expr) {
            return error;
        }

        (
            *expr,
            quote! {
                num_macro_fn::<#ty>()
            },
        )
    } else {
        (
            input,
            quote! {
                num_macro_fn()
            },
        )
    };

    quote! {
        {
            const MACRO_INPUT: i128 = #expr;

            {
                fn num_macro_fn<NumMacroType: crate::Num>() -> NumMacroType {
                    if const {
                        if let Some(min) = NumMacroType::MIN_CONSTANT {
                            if MACRO_INPUT < min {
                                panic!("number too small for given type")
                            }
                        }

                        if let Some(max) = NumMacroType::MAX_CONSTANT {
                            if MACRO_INPUT > max {
                                panic!("number too large for given type")
                            }
                        }

                        true
                    } {
                        unsafe { NumMacroType::int_constant(MACRO_INPUT) }
                    } else {
                        unreachable!()
                    }
                }

                #fn_call
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn crate_float(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Expr);

    let (expr, fn_call) = if let Expr::Cast(ExprCast {
        attrs: _,
        expr,
        as_token: _,
        ty,
    }) = input
    {
        fn verify_expr(expr: &Expr) -> Result<(), proc_macro::TokenStream> {
            match expr {
                Expr::Lit(_) => Ok(()),
                Expr::Unary(expr) => verify_expr(&expr.expr),
                _ => Err(Error::new(expr.span(), "`( ... ) required`")
                    .to_compile_error()
                    .into()),
            }
        }

        if let Err(error) = verify_expr(&expr) {
            return error;
        }

        (
            *expr,
            quote! {
                num_macro_fn::<#ty>()
            },
        )
    } else {
        (
            input,
            quote! {
                num_macro_fn()
            },
        )
    };

    quote! {
        {
            const MACRO_INPUT: f64 = #expr;

            {
                fn num_macro_fn<NumMacroType: crate::Num>() -> NumMacroType {
                    if const {
                        if !NumMacroType::ALLOW_FLOAT_CONSTANTS {
                            panic!("float constants not allowed for given type")
                        }

                        if let Some(min) = NumMacroType::MIN_CONSTANT {
                            if MACRO_INPUT < min as f64 {
                                panic!("number too small for given type")
                            }
                        }

                        if let Some(max) = NumMacroType::MAX_CONSTANT {
                            if MACRO_INPUT > max as f64 {
                                panic!("number too large for given type")
                            }
                        }

                        true
                    } {
                        unsafe { NumMacroType::float_constant(MACRO_INPUT) }
                    } else {
                        unreachable!()
                    }
                }

                #fn_call
            }
        }
    }
    .into()
}
