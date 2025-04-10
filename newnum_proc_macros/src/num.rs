use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Lit, Token};

pub fn num(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num_macro_helper(
        "from_int_literal",
        "from_float_literal",
        quote! { ::newnum },
        input,
    )
}

pub fn num_approx(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num_macro_helper(
        "approx_from_int_literal",
        "approx_from_float_literal",
        quote! { ::newnum },
        input,
    )
}

pub fn internal_num(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    num_macro_helper(
        "from_int_literal",
        "from_float_literal",
        quote! { crate },
        input,
    )
}

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
