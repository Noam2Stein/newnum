use proc_macro2::{Literal, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Data, Error, Type};

pub fn derive_rhs_map_fields(
    input: &syn::DeriveInput,
    trait_ident: &str,
    rhs_ident: &str,
    mut map: impl FnMut(&TokenStream, &TokenStream, &Type) -> TokenStream,
) -> proc_macro2::TokenStream {
    let rhs_ident = format_ident!("{rhs_ident}");

    match &input.data {
        Data::Struct(data) => {
            let members = data.fields.members();
            let member_values = data.fields.iter().enumerate().map(|(i, field)| {
                let (member, rhs_member) = if let Some(ident) = &field.ident {
                    (quote! { self.#ident }, quote! { #rhs_ident.#ident })
                } else {
                    let i = Literal::usize_unsuffixed(i);

                    (quote! { self.#i }, quote! { #rhs_ident.#i })
                };

                map(&member, &rhs_member, &field.ty)
            });

            quote! {
                Self {#(
                    #members: #member_values,
                )*}
            }
        }
        Data::Enum(_) => Error::new(
            Span::call_site(),
            format!("`{trait_ident}` cannot be derived for enums"),
        )
        .to_compile_error()
        .into(),
        Data::Union(_) => Error::new(
            Span::call_site(),
            format!("`{trait_ident}` cannot be derived for unions"),
        )
        .to_compile_error()
        .into(),
    }
}
