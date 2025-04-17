use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::derive_split_generics;

pub fn whole_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Whole");

    quote! {
        impl #impl_generics ::newnum::Round for #type_ident #ty_generics #where_clause {
            fn round(self) -> Self {
                self
            }
            fn floor(self) -> Self {
                self
            }
            fn ceil(self) -> Self {
                self
            }
            fn trunc(self) -> Self {
                self
            }
            fn atrunc(self) -> Self {
                self
            }
            fn fract(self) -> Self {
                <Self as ::newnum::Zero>::zero()
            }
        }

        impl #impl_generics ::newnum::WholeEquivalent for #type_ident #ty_generics #where_clause {
            type Whole = Self;

            fn iround(self) -> Self::Whole {
                self
            }
            fn ifloor(self) -> Self::Whole {
                self
            }
            fn iceil(self) -> Self::Whole {
                self
            }
            fn itrunc(self) -> Self::Whole {
                self
            }
            fn iatrunc(self) -> Self::Whole {
                self
            }
        }
    }
    .into()
}
