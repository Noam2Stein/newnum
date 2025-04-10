use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{Data, Error, Fields, Type};

pub fn derive_map_fields(
    input: &syn::DeriveInput,
    trait_ident: &str,
    mut map: impl FnMut(&TokenStream, &Type) -> TokenStream,
) -> proc_macro2::TokenStream {
    match &input.data {
        Data::Struct(data) => {
            let members = data.fields.members();
            let member_values = data.fields.iter().enumerate().map(|(i, field)| {
                let member = if let Some(ident) = &field.ident {
                    quote! { self.#ident }
                } else {
                    let i = Literal::usize_unsuffixed(i);
                    quote! { self.#i }
                };

                map(&member, &field.ty)
            });

            quote! {
                Self {#(
                    #members: #member_values,
                )*}
            }
        }
        Data::Enum(data) => {
            let variant_idents = data
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect::<Vec<_>>();

            let variant_fields = data.variants.iter().map(|variant| match &variant.fields {
                syn::Fields::Named(fields) => {
                    let field_idents = fields.named.iter().map(|f| &f.ident);

                    quote! { { #(#field_idents), * } }
                }
                syn::Fields::Unnamed(fields) => {
                    let field_idents = fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format_ident!("field_{i}"));

                    quote! { (#(#field_idents), *) }
                }
                syn::Fields::Unit => quote! {},
            });

            let variant_members = data
                .variants
                .iter()
                .map(|variant| variant.fields.members().collect::<Vec<_>>());

            let variant_member_values = data.variants.iter().map(|variant| match &variant.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|field| map(&field.ident.to_token_stream(), &field.ty))
                    .collect::<Vec<_>>(),
                Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, field)| map(&format_ident!("field_{i}").to_token_stream(), &field.ty))
                    .collect::<Vec<_>>(),
                Fields::Unit => Vec::new(),
            });

            quote! {#(
                #variant_idents #variant_fields => Self::#variant_idents {#(
                    #variant_members: #variant_member_values,
                )*},
            )*}
        }
        Data::Union(_) => Error::new(
            Span::call_site(),
            format!("`{trait_ident}` cannot be derived for unions"),
        )
        .to_compile_error()
        .into(),
    }
}
