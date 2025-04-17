use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_map_fields, derive_split_generics};

pub fn round_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Round");

    let round_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::round(#field)
        }
    });
    let floor_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::floor(#field)
        }
    });
    let ceil_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::ceil(#field)
        }
    });
    let trunc_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::trunc(#field)
        }
    });
    let atrunc_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::atrunc(#field)
        }
    });
    let fract_output = derive_map_fields(&input, "Round", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Round>::fract(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Round for #type_ident #ty_generics #where_clause {
            fn round(self) -> Self {
                #round_output
            }
            fn floor(self) -> Self {
                #floor_output
            }
            fn ceil(self) -> Self {
                #ceil_output
            }
            fn trunc(self) -> Self {
                #trunc_output
            }
            fn atrunc(self) -> Self {
                #atrunc_output
            }
            fn fract(self) -> Self {
                #fract_output
            }
        }
    }
    .into()
}
