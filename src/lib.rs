mod generator;

use proc_macro::TokenStream;
use quote::quote_spanned;
use syn::{Item, spanned::Spanned};

use crate::generator::gen_negated_function;

#[proc_macro_attribute]
pub fn negate(_: TokenStream, input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("failed to parse input into `syn::Item`");

    match item {
        Item::Fn(function) => gen_negated_function(function),
        other => {
            let err = quote_spanned! {
                other.span() =>
                compile_error!("`negate` can only be applied to functions.");
            };

            err.into()
        }
    }
}
