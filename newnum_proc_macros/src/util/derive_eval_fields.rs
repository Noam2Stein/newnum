use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{spanned::Spanned, Data, Error, Type};

pub fn derive_eval_fields(
    input: &syn::DeriveInput,
    trait_ident: &str,
    variant_attr: &str,
    mut eval: impl FnMut(&Type) -> TokenStream,
) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let members = data.fields.members();
            let member_values = data.fields.iter().map(|field| eval(&field.ty));

            quote! {
                Self {#(
                    #members: #member_values,
                )*}
            }
        }
        Data::Enum(data) => {
            let selected_variants = data
                .variants
                .iter()
                .filter(|variant| {
                    variant
                        .attrs
                        .iter()
                        .any(|attr| attr.path().is_ident(variant_attr))
                })
                .collect::<Vec<_>>();

            if selected_variants.len() != 1 {
                return Error::new(
                    selected_variants.get(2).span(),
                    format!("`{variant_attr}` must be used on exactly one variant"),
                )
                .to_compile_error()
                .into();
            }

            let variant = selected_variants.into_iter().next().unwrap();

            let variant_ident = &variant.ident;

            let members = variant.fields.members();
            let member_values = variant.fields.iter().map(|field| eval(&field.ty));

            quote! {
                Self::#variant_ident {#(
                    #members: #member_values,
                )*}
            }
        }
        Data::Union(_) => Error::new(
            Span::call_site(),
            format!("`{trait_ident}` cannot be derived for unions"),
        )
        .to_compile_error()
        .into(),
    }
}
