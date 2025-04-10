use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_map_fields, derive_split_generics};

pub fn min_max_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "MinMax");

    let min_output = derive_map_fields(&input, "MinMax", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::MinMax>::min(#field)
        }
    });
    let max_output = derive_map_fields(&input, "MinMax", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::MinMax>::max(#field)
        }
    });
    let clamp_output = derive_map_fields(&input, "MinMax", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::MinMax>::clamp(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::MinMax for #type_ident #ty_generics #where_clause {
            fn min(self) -> Self {
                #min_output
            }
            fn max(self) -> Self {
                #max_output
            }
            fn clamp(self) -> Self {
                #clamp_output
            }
        }
    }
    .into()
}
