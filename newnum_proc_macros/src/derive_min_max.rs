use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

use crate::util::derive_split_generics;

pub fn min_max_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "MinMax");

    let (min_output, max_output, clamp_output) = if input
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident("flat_minmax"))
    {
        (
            quote! {
                if self < other {
                    self
                } else {
                    other
                }
            },
            quote! {
                if self > other {
                    self
                } else {
                    other
                }
            },
            quote! {
                if self > max {
                    max
                } else if self < min {
                    min
                } else {
                    self
                }
            },
        )
    } else {
        match &input.data {
            Data::Struct(data) => (
                {
                    let members = data.fields.members();
                    let field_types = data.fields.iter().map(|field| &field.ty);

                    quote! {
                        Self {#(
                            #members: <#field_types as ::newnum::MinMax>::min(self.#members, other.#members),
                        )*}
                    }
                },
                {
                    let members = data.fields.members();
                    let field_types = data.fields.iter().map(|field| &field.ty);

                    quote! {
                        Self {#(
                            #members: <#field_types as ::newnum::MinMax>::max(self.#members, other.#members),
                        )*}
                    }
                },
                {
                    let members = data.fields.members();
                    let field_types = data.fields.iter().map(|field| &field.ty);

                    quote! {
                        Self {#(
                            #members: <#field_types as ::newnum::MinMax>::clamp(self.#members, min.#members, max.#members),
                        )*}
                    }
                },
            ),
            Data::Enum(_) => {
                return Error::new(Span::call_site(), "`MinMax` cannot be derived for enums")
                    .into_compile_error()
                    .into()
            }
            Data::Union(_) => {
                return Error::new(Span::call_site(), "`MinMax` cannot be derived for unions")
                    .into_compile_error()
                    .into()
            }
        }
    };

    quote! {
        impl #impl_generics ::newnum::MinMax for #type_ident #ty_generics #where_clause {
            fn min(self, other: Self) -> Self {
                #min_output
            }
            fn max(self, other: Self) -> Self {
                #max_output
            }
            fn clamp(self, min: Self, max: Self) -> Self {
                #clamp_output
            }
        }
    }
    .into()
}
