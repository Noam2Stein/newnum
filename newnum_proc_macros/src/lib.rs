use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Expr, ExprCast, Fields, Ident,
    Lit, LitInt,
};

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

#[proc_macro_derive(Num)]
pub fn derive_num(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::Num for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Prim)]
pub fn derive_prim(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::Prim for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(SignedPrim)]
pub fn derive_signed_prim(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::SignedPrim for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(UnsignedPrim)]
pub fn derive_unsigned_prim(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::UnsignedPrim for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Int)]
pub fn derive_int(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::Int for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(UInt)]
pub fn derive_uint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::UInt for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(SInt)]
pub fn derive_sint(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::SInt for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Float)]
pub fn derive_float(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::Float for #ident #ty_generics where #where_clause {}
    }
    .into()
}

#[proc_macro_derive(Whole)]
pub fn derive_whole(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::WholeEquivalent for #ident #ty_generics where #where_clause {
            type Whole = Self;
        }

        impl #impl_generics newnum::Round for #ident #ty_generics where #where_clause {
            #[inline(always)]
            fn round(self) -> Self {
                self
            }

            #[inline(always)]
            fn floor(self) -> Self {
                self
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                self
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                self
            }

            #[inline(always)]
            fn atrunc(self) -> Self {
                self
            }
        }

        impl #impl_generics newnum::IRound for #ident #ty_generics where #where_clause {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iatrunc(self) -> Self::Whole {
                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(Floating)]
pub fn derive_floating(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::FloatingEquivalent for #ident #ty_generics where #where_clause {
            type Floating = Self;

            #[inline(always)]
            fn floating(self) -> Self::Floating {
                self
            }
        }
    }
    .into()
}

#[proc_macro_derive(Round)]
pub fn derive_round(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let (round, floor, ceil, trunc, atrunc) = match input.data {
        Data::Enum(data) => {
            todo!()
        }
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let round_fields = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    quote! {
                        field_ident: self.#field_ident.round()
                    }
                });
                let floor_fields = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    quote! {
                        field_ident: self.#field_ident.floor()
                    }
                });
                let ceil_fields = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    quote! {
                        field_ident: self.#field_ident.ceil()
                    }
                });
                let trunc_fields = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    quote! {
                        field_ident: self.#field_ident.trunc()
                    }
                });
                let atrunc_fields = fields.named.iter().map(|field| {
                    let field_ident = field.ident.as_ref().unwrap();
                    quote! {
                        field_ident: self.#field_ident.atrunc()
                    }
                });

                (
                    quote! {
                        Self { #(#round_fields, )* }
                    },
                    quote! {
                        Self { #(#floor_fields, )* }
                    },
                    quote! {
                        Self { #(#ceil_fields, )* }
                    },
                    quote! {
                        Self { #(#trunc_fields, )* }
                    },
                    quote! {
                        Self { #(#atrunc_fields, )* }
                    },
                )
            }
            Fields::Unit => (
                quote! { Self },
                quote! { Self },
                quote! { Self },
                quote! { Self },
                quote! { Self },
            ),
            Fields::Unnamed(fields) => {
                let fields_iter = (0..fields.unnamed.len()).zip(fields.unnamed.iter());

                let round_fields = fields_iter.clone().map(|(field_index, field)| {
                    let field_index = Lit::Int(LitInt::new(&field_index.to_string(), field.span()));
                    quote! {
                        self.#field_index.round()
                    }
                });
                let floor_fields = fields_iter.clone().map(|(field_index, field)| {
                    let field_index = Lit::Int(LitInt::new(&field_index.to_string(), field.span()));
                    quote! {
                        self.#field_index.floor()
                    }
                });
                let ceil_fields = fields_iter.clone().map(|(field_index, field)| {
                    let field_index = Lit::Int(LitInt::new(&field_index.to_string(), field.span()));
                    quote! {
                        self.#field_index.ceil()
                    }
                });
                let trunc_fields = fields_iter.clone().map(|(field_index, field)| {
                    let field_index = Lit::Int(LitInt::new(&field_index.to_string(), field.span()));
                    quote! {
                        self.#field_index.trunc()
                    }
                });
                let atrunc_fields = fields_iter.clone().map(|(field_index, field)| {
                    let field_index = Lit::Int(LitInt::new(&field_index.to_string(), field.span()));
                    quote! {
                        self.#field_index.atrunc()
                    }
                });

                (
                    quote! {
                        Self(#(#round_fields), *)
                    },
                    quote! {
                        Self(#(#floor_fields), *)
                    },
                    quote! {
                        Self(#(#ceil_fields), *)
                    },
                    quote! {
                        Self(#(#trunc_fields), *)
                    },
                    quote! {
                        Self(#(#atrunc_fields), *)
                    },
                )
            }
        },
        Data::Union(_) => (
            quote! {
                compiler_error!("`Round` cannot be derived by unions");
                todo!()
            },
            quote! {
                todo!()
            },
            quote! {
                todo!()
            },
            quote! {
                todo!()
            },
            quote! {
                todo!()
            },
        ),
    };

    quote! {
        impl #impl_generics newnum::Round for #ident #ty_generics where #where_clause {
            #[inline(always)]
            fn round(self) -> Self {
                #round
            }

            #[inline(always)]
            fn floor(self) -> Self {
                #floor
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                #ceil
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                #trunc
            }

            #[inline(always)]
            fn atrunc(self) -> Self {
                #atrunc
            }
        }
    }
    .into()
}

fn impl_empty_trait(input: &DeriveInput, trait_ident: &str) -> TokenStream {
    let ident = &input.ident;
    let trait_ident = Ident::new(trait_ident, input.span());
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::#trait_ident for #ident #ty_generics where #where_clause {}
    }
}

fn impl_whole(input: &DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        impl #impl_generics newnum::WholeEquivalent for #ident #ty_generics where #where_clause {
            type Whole = Self;
        }

        impl #impl_generics newnum::Round for #ident #ty_generics where #where_clause {
            #[inline(always)]
            fn round(self) -> Self {
                self
            }

            #[inline(always)]
            fn floor(self) -> Self {
                self
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                self
            }

            #[inline(always)]
            fn trunc(self) -> Self {
                self
            }

            #[inline(always)]
            fn atrunc(self) -> Self {
                self
            }
        }

        impl #impl_generics newnum::IRound for #ident #ty_generics where #where_clause {
            #[inline(always)]
            fn iround(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn ifloor(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iceil(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn itrunc(self) -> Self::Whole {
                self
            }

            #[inline(always)]
            fn iatrunc(self) -> Self::Whole {
                self
            }
        }
    }
}
