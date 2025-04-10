use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

use crate::util::{derive_map_fields, derive_split_generics};

pub fn trig_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Trig");

    let sin_output = derive_map_fields(&input, "Trig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Trig>::sin(#field)
        }
    });
    let cos_output = derive_map_fields(&input, "Trig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Trig>::cos(#field)
        }
    });
    let tan_output = derive_map_fields(&input, "Trig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Trig>::tan(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Trig for #type_ident #ty_generics #where_clause {
            type Output = Self;

            fn sin(self) -> Self::Output {
                #sin_output
            }
            fn cos(self) -> Self::Output {
                #cos_output
            }
            fn tan(self) -> Self::Output {
                #tan_output
            }
        }
    }
    .into()
}

pub fn atrig_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "ATrig");

    let asin_output = derive_map_fields(&input, "ATrig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::ATrig>::asin(#field)
        }
    });
    let acos_output = derive_map_fields(&input, "ATrig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::ATrig>::acos(#field)
        }
    });
    let atan_output = derive_map_fields(&input, "ATrig", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::ATrig>::atan(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::ATrig for #type_ident #ty_generics #where_clause {
            type Output = Self;

            fn asin(self) -> Self::Output {
                #asin_output
            }
            fn acos(self) -> Self::Output {
                #acos_output
            }
            fn atan(self) -> Self::Output {
                #atan_output
            }
        }
    }
    .into()
}

pub fn hyper_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Hyper");

    let sinh_output = derive_map_fields(&input, "Hyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Hyper>::sinh(#field)
        }
    });
    let cosh_output = derive_map_fields(&input, "Hyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Hyper>::cosh(#field)
        }
    });
    let tanh_output = derive_map_fields(&input, "Hyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::Hyper>::tanh(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Hyper for #type_ident #ty_generics #where_clause {
            type Output = Self;

            fn sinh(self) -> Self::Output {
                #sinh_output
            }
            fn cosh(self) -> Self::Output {
                #cosh_output
            }
            fn tanh(self) -> Self::Output {
                #tanh_output
            }
        }
    }
    .into()
}

pub fn ahyper_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "AHyper");

    let asinh_output = derive_map_fields(&input, "AHyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::AHyper>::asinh(#field)
        }
    });
    let acosh_output = derive_map_fields(&input, "AHyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::AHyper>::acosh(#field)
        }
    });
    let atanh_output = derive_map_fields(&input, "AHyper", |field, field_type| {
        quote_spanned! {
            field_type.span() =>
            <#field_type as ::newnum::AHyper>::atanh(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::AHyper for #type_ident #ty_generics #where_clause {
            type Output = Self;

            fn asinh(self) -> Self::Output {
                #asinh_output
            }
            fn acosh(self) -> Self::Output {
                #acosh_output
            }
            fn atanh(self) -> Self::Output {
                #atanh_output
            }
        }
    }
    .into()
}
