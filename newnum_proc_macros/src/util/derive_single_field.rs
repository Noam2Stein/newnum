use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, Data, DeriveInput, Error, Fields, Type};

pub fn derive_map_single_field_ref(
    input: &DeriveInput,
    trait_ident: &str,
    mut map: impl FnMut(&TokenStream, &Type) -> TokenStream,
) -> TokenStream {
    match &input.data {
        Data::Struct(data) => {
            if data.fields.len() != 1 {
                return Error::new(
                    input.span(),
                    format!("`{trait_ident}` expects a single field",),
                )
                .to_compile_error();
            };

            let field = data.fields.iter().next().unwrap();
            let member = data.fields.members().next().unwrap();

            map(&quote! { &self.#member }, &field.ty)
        }
        Data::Enum(data) => {
            for variant in &data.variants {
                if variant.fields.len() != 1 {
                    return Error::new(
                        variant.span(),
                        format!("`{trait_ident}` expects a single field",),
                    )
                    .to_compile_error();
                };
            }

            let variant_idents = data
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect::<Vec<_>>();

            let variant_field = data.variants.iter().map(|variant| match &variant.fields {
                Fields::Named(fields) => {
                    let field_ident = fields.named.iter().next().unwrap().ident.as_ref();

                    quote! { { #field_ident: field } }
                }
                Fields::Unnamed(_) => {
                    quote! { (field) }
                }
                Fields::Unit => unreachable!(),
            });

            let variant_outputs = data.variants.iter().map(|variant| {
                let field_type = &variant.fields.iter().next().unwrap().ty;

                map(&quote! { field }, field_type)
            });

            quote! {
                match self {#(
                    Self::#variant_idents #variant_field => #variant_outputs,
                )*}
            }
        }
        Data::Union(_) => {
            return Error::new(
                input.span(),
                format!("`{trait_ident}` cannot be derived for unions",),
            )
            .to_compile_error();
        }
    }
}
