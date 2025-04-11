use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_map_fields, derive_split_generics};

pub fn trunc_root_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "TruncRoot");

    let trunc_sqrt_output = derive_map_fields(&input, "TruncRoot", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::TruncRoot>::trunc_sqrt(#field)
        }
    });
    let trunc_cbrt_output = derive_map_fields(&input, "TruncRoot", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::TruncRoot>::trunc_cbrt(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::TruncRoot for #type_ident #ty_generics #where_clause {
            fn trunc_sqrt(self) -> Self {
                #trunc_sqrt_output
            }
            fn trunc_cbrt(self) -> Self {
                #trunc_cbrt_output
            }
        }
    }
    .into()
}

pub fn root_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Root");

    let sqrt_output = derive_map_fields(&input, "Root", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Root>::sqrt(#field)
        }
    });
    let cbrt_output = derive_map_fields(&input, "Root", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Root>::cbrt(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Root for #type_ident #ty_generics #where_clause {
            fn sqrt(self) -> Self {
                #sqrt_output
            }
            fn cbrt(self) -> Self {
                #cbrt_output
            }
        }
    }
    .into()
}
