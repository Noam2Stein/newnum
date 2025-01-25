use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, Lit, LitInt};

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
