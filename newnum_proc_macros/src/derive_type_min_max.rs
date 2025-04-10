use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_eval_fields, derive_split_generics};

pub fn type_min_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "TypeMin");

    let output = derive_eval_fields(&input, "TypeMin", "type_min", |field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::TypeMin>::type_min()
        }
    });

    quote! {
        impl #impl_generics ::newnum::TypeMin for #type_ident #ty_generics #where_clause {
            fn type_min() -> Self {
                #output
            }
        }
    }
    .into()
}

pub fn type_max_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "TypeMax");

    let output = derive_eval_fields(&input, "TypeMax", "type_max", |field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::TypeMax>::type_max()
        }
    });

    quote! {
        impl #impl_generics ::newnum::TypeMax for #type_ident #ty_generics #where_clause {
            fn type_max() -> Self {
                #output
            }
        }
    }
    .into()
}
