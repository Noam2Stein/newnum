use quote::quote;
use syn::{parse_macro_input, DeriveInput};

use crate::util::{
    derive_eval_fields, derive_map_fields, derive_map_single_field_ref, derive_split_generics,
};

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

pub fn positive_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Positive");

    let abs_output = derive_map_fields(&input, "Positive", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Positive>::abs(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Positive for #type_ident #ty_generics #where_clause {
            fn abs(self) -> Self {
                #abs_output
            }
        }
    }
    .into()
}
pub fn negative_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Negative");

    let neg_abs_output = derive_map_fields(&input, "Negative", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Negative>::neg_abs(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Negative for #type_ident #ty_generics #where_clause {
            fn neg_abs(self) -> Self {
                #neg_abs_output
            }
        }
    }
    .into()
}
pub fn zero_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Zero");

    let zero_output = derive_eval_fields(&input, "Zero", "zero", |field_type| {
        quote! {
            <#field_type as ::newnum::Zero>::zero()
        }
    });

    quote! {
        impl #impl_generics ::newnum::Zero for #type_ident #ty_generics #where_clause {
            fn zero() -> Self {
                #zero_output
            }
        }
    }
    .into()
}

pub fn not_positive_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "NotPositive");

    quote! {
        impl #impl_generics ::newnum::NotPositive for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn not_negative_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "NotNegative");

    quote! {
        impl #impl_generics ::newnum::NotNegative for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn not_zero_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "NotZero");

    quote! {
        impl #impl_generics ::newnum::NotZero for #type_ident #ty_generics #where_clause {}
    }
    .into()
}

pub fn always_positive_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Positive");

    quote! {
        impl #impl_generics ::newnum::Positive for #type_ident #ty_generics #where_clause {
            fn abs(self) -> Self {
                self
            }
        }

        impl #impl_generics ::newnum::NotNegative for #type_ident #ty_generics #where_clause {}
        impl #impl_generics ::newnum::NotZero for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn always_negative_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Negative");

    quote! {
        impl #impl_generics ::newnum::Negative for #type_ident #ty_generics #where_clause {
            fn neg_abs(self) -> Self {
                self
            }
        }

        impl #impl_generics ::newnum::NotPositive for #type_ident #ty_generics #where_clause {}
        impl #impl_generics ::newnum::NotZero for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn always_zero_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Zero");

    let zero_output = derive_eval_fields(&input, "Zero", "zero", |field_type| {
        quote! {
            <#field_type as ::newnum::Zero>::zero()
        }
    });

    quote! {
        impl #impl_generics ::newnum::Zero for #type_ident #ty_generics #where_clause {
            fn zero() -> Self {
                #zero_output
            }
        }

        impl #impl_generics ::newnum::NotPositive for #type_ident #ty_generics #where_clause {}
        impl #impl_generics ::newnum::NotNegative for #type_ident #ty_generics #where_clause {}
    }
    .into()
}

pub fn positive_or_zero_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Positive");

    let zero_output = derive_eval_fields(&input, "Zero", "zero", |field_type| {
        quote! {
            <#field_type as ::newnum::Zero>::zero()
        }
    });

    quote! {
        impl #impl_generics ::newnum::Positive for #type_ident #ty_generics #where_clause {
            fn abs(self) -> Self {
                self
            }
        }
        impl #impl_generics ::newnum::Zero for #type_ident #ty_generics #where_clause {
            fn zero() -> Self {
                #zero_output
            }
        }

        impl #impl_generics ::newnum::NotNegative for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn negative_or_zero_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Negative");

    let zero_output = derive_eval_fields(&input, "Zero", "zero", |field_type| {
        quote! {
            <#field_type as ::newnum::Zero>::zero()
        }
    });

    quote! {
        impl #impl_generics ::newnum::Negative for #type_ident #ty_generics #where_clause {
            fn neg_abs(self) -> Self {
                self
            }
        }
        impl #impl_generics ::newnum::Zero for #type_ident #ty_generics #where_clause {
            fn zero() -> Self {
                #zero_output
            }
        }

        impl #impl_generics ::newnum::NotPositive for #type_ident #ty_generics #where_clause {}
    }
    .into()
}
pub fn positive_or_negative_derive_macro(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Negative");

    let abs_output = derive_map_fields(&input, "Positive", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Positive>::abs(#field)
        }
    });
    let neg_abs_output = derive_map_fields(&input, "Negative", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Negative>::neg_abs(#field)
        }
    });

    quote! {
        impl #impl_generics ::newnum::Positive for #type_ident #ty_generics #where_clause {
            fn abs(self) -> Self {
                #abs_output
            }
        }
        impl #impl_generics ::newnum::Negative for #type_ident #ty_generics #where_clause {
            fn neg_abs(self) -> Self {
                #neg_abs_output
            }
        }

        impl #impl_generics ::newnum::NotZero for #type_ident #ty_generics #where_clause {}
    }
    .into()
}

pub fn fully_signed_derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_split_generics(&input, "Negative");

    let abs_output = derive_map_fields(&input, "Positive", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Positive>::abs(#field)
        }
    });
    let neg_abs_output = derive_map_fields(&input, "Negative", |field, field_type| {
        quote! {
            <#field_type as ::newnum::Negative>::neg_abs(#field)
        }
    });
    let zero_output = derive_eval_fields(&input, "Zero", "zero", |field_type| {
        quote! {
            <#field_type as ::newnum::Zero>::zero()
        }
    });

    quote! {
        impl #impl_generics ::newnum::Positive for #type_ident #ty_generics #where_clause {
            fn abs(self) -> Self {
                #abs_output
            }
        }
        impl #impl_generics ::newnum::Negative for #type_ident #ty_generics #where_clause {
            fn neg_abs(self) -> Self {
                #neg_abs_output
            }
        }
        impl #impl_generics ::newnum::Zero for #type_ident #ty_generics #where_clause {
            fn zero() -> Self {
                #zero_output
            }
        }
    }
    .into()
}
