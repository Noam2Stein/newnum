use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Error};

use crate::derive_bound::derive_split_generics;

//#[proc_macro_derive(Sign, attributes(derive_bound))]
pub fn sign_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;

    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Sign");

    let items = match &input.data {
        Data::Struct(data) => {
            if data.fields.len() == 1 {
                let member = data.fields.members().next().unwrap();
                let field_type = &data.fields.iter().next().unwrap().ty;

                quote! {
                    type BoolMapped = bool;

                    fn is_positive(&self) -> Self::BoolMapped {
                        <#field_type as ::newnum::Sign>::is_positive(&self.#member)
                    }
                    fn is_negative(&self) -> Self::BoolMapped {
                        <#field_type as ::newnum::Sign>::is_negative(&self.#member)
                    }

                    fn is_zero(&self) -> Self::BoolMapped {
                        <#field_type as ::newnum::Sign>::is_zero(&self.#member)
                    }

                    fn is_bin_positive(&self) -> Self::BoolMapped {
                        <#field_type as ::newnum::Sign>::is_bin_positive(&self.#member)
                    }
                    fn is_bin_negative(&self) -> Self::BoolMapped {
                        <#field_type as ::newnum::Sign>::is_bin_negative(&self.#member)
                    }
                }
            } else {
                return Error::new(Span::call_site(), "expected a single field")
                    .to_compile_error()
                    .into();
            }
        }
        Data::Enum(data) => {
            for variant in &data.variants {
                if variant.fields.len() != 1 {
                    return Error::new(variant.span(), "`Sign` expects a single field")
                        .to_compile_error()
                        .into();
                }
            }

            let variant_idents = data
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect::<Vec<_>>();

            let variant_field_idents = data
                .variants
                .iter()
                .map(|variant| &variant.fields)
                .collect::<Vec<_>>();

            let variant_field_types = data
                .variants
                .iter()
                .map(|variant| &variant.fields.iter().next().unwrap().ty)
                .collect::<Vec<_>>();

            quote! {
                type BoolMapped = bool;

                fn is_positive(&self) -> Self::BoolMapped {
                    match self {
                        #(Self::#variant_idents { #variant_field_idents: field } => <#variant_field_types as ::newnum::Sign>::is_positive(field),)*
                    }
                }
                fn is_negative(&self) -> Self::BoolMapped {
                    match self {
                        #(Self::#variant_idents { #variant_field_idents: field } => <#variant_field_types as ::newnum::Sign>::is_negative(field),)*
                    }
                }

                fn is_zero(&self) -> Self::BoolMapped {
                    match self {
                        #(Self::#variant_idents { #variant_field_idents: field } => <#variant_field_types as ::newnum::Sign>::is_zero(field),)*
                    }
                }

                fn is_bin_positive(&self) -> Self::BoolMapped {
                    match self {
                        #(Self::#variant_idents { #variant_field_idents: field } => <#variant_field_types as ::newnum::Sign>::is_bin_positive(field),)*
                    }
                }
                fn is_bin_negative(&self) -> Self::BoolMapped {
                    match self {
                        #(Self::#variant_idents { #variant_field_idents: field } => <#variant_field_types as ::newnum::Sign>::is_bin_negative(field),)*
                    }
                }
            }
        }
        Data::Union(_) => {
            return Error::new(Span::call_site(), "`Sign` cannot be derived for unions")
                .to_compile_error()
                .into()
        }
    };

    quote! {
        impl #impl_generics ::newnum::Sign for #type_ident #ty_generics #where_clause {
            #items
        }
    }
    .into()
}
