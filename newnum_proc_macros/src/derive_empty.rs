use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::derive_attributes::derive_split_generics;

pub fn num_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Num");

    quote! {
        impl #impl_generics ::newnum::Num for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
