use derive_syn_parse::Parse;
use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, DeriveInput, Ident, ImplGenerics, Token, TypeGenerics, WhereClause,
    WherePredicate,
};

pub fn derive_split_generics<'a>(
    input: &'a DeriveInput,
    trait_ident: &str,
) -> (ImplGenerics<'a>, TypeGenerics<'a>, Option<WhereClause>) {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut where_clause = where_clause.cloned();

    let derive_bounds = input
        .attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("derive_bound") {
                let DeriveBound {
                    target_derive,
                    bounds,
                } = attr.parse_args().ok()?;

                if target_derive.to_string() == trait_ident {
                    Some(bounds.into_iter())
                } else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    if !derive_bounds.is_empty() {
        if let Some(where_clause) = &mut where_clause {
            where_clause.predicates.extend(derive_bounds);
        } else {
            where_clause = Some(WhereClause {
                where_token: Token![where](Span::call_site()),
                predicates: Punctuated::from_iter(derive_bounds),
            });
        }
    }

    (impl_generics, ty_generics, where_clause)
}

#[derive(Parse)]
struct DeriveBound {
    target_derive: Ident,
    #[prefix(Token![;])]
    #[call(Punctuated::parse_terminated)]
    bounds: Punctuated<WherePredicate, Token![,]>,
}
