use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_rhs_map_fields, derive_split_generics};

pub fn abs_diff_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "AbsDiff");

    let output = derive_rhs_map_fields(&input, "AbsDiff", "rhs", |field, rhs_field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::AbsDiff>::abs_diff(#field, #rhs_field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::AbsDiff for #type_ident #ty_generics #where_clause {
            fn abs_diff(self, rhs: Self) -> Self::Output {
                #output
            }
        }
    }
    .into()
}
