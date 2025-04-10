use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::{derive_map_single_field_ref, derive_split_generics};

pub fn sign_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Sign");

    let is_positive_output = derive_map_single_field_ref(&input, "Sign", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Sign>::is_positive(#field)
        }
    });

    let is_negative_output = derive_map_single_field_ref(&input, "Sign", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Sign>::is_negative(#field)
        }
    });
    let is_zero_output = derive_map_single_field_ref(&input, "Sign", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Sign>::is_zero(#field)
        }
    });

    let is_bin_positive_output =
        derive_map_single_field_ref(&input, "Sign", |field, field_type| {
            quote! {
                <#field_type as ::newnum::Sign>::is_bin_positive(#field)
            }
        });
    let is_bin_negative_output =
        derive_map_single_field_ref(&input, "Sign", |field, field_type| {
            quote! {
                <#field_type as ::newnum::Sign>::is_bin_negative(#field)
            }
        });

    quote! {
        impl #impl_generics ::newnum::Sign for #type_ident #ty_generics #where_clause {
            type BoolMapped = bool;

            fn is_positive(&self) -> Self::BoolMapped {
                #is_positive_output
            }
            fn is_negative(&self) -> Self::BoolMapped {
                #is_negative_output
            }

            fn is_zero(&self) -> Self::BoolMapped {
                #is_zero_output
            }

            fn is_bin_positive(&self) -> Self::BoolMapped {
                #is_bin_positive_output
            }
            fn is_bin_negative(&self) -> Self::BoolMapped {
                #is_bin_negative_output
            }
        }
    }
    .into()
}
